use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Burn};
use solana_program::clock::Clock;

use crate::state::*;
use crate::SnitchError;
use crate::RoundResolved;

#[derive(Accounts)]
pub struct ResolveRound<'info> {
    #[account(mut)]
    pub resolver: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        mut,
        constraint = interrogation.key() == game_state.active_interrogation.unwrap_or(Pubkey::default()) @ SnitchError::NoActiveRound,
    )]
    pub interrogation: Account<'info, Interrogation>,
    
    /// CHECK: Player A profile
    #[account(
        mut,
        seeds = [b"player", interrogation.player_a.as_ref()],
        bump,
    )]
    pub player_a_profile: Account<'info, PlayerProfile>,
    
    /// CHECK: Player B profile  
    #[account(
        mut,
        seeds = [b"player", interrogation.player_b.as_ref()],
        bump,
    )]
    pub player_b_profile: Account<'info, PlayerProfile>,
    
    /// Player A's token account
    #[account(
        mut,
        constraint = player_a_token.owner == interrogation.player_a @ SnitchError::Unauthorized,
        constraint = player_a_token.mint == game_state.token_mint @ SnitchError::Unauthorized,
    )]
    pub player_a_token: Account<'info, TokenAccount>,
    
    /// Player B's token account
    #[account(
        mut,
        constraint = player_b_token.owner == interrogation.player_b @ SnitchError::Unauthorized,
        constraint = player_b_token.mint == game_state.token_mint @ SnitchError::Unauthorized,
    )]
    pub player_b_token: Account<'info, TokenAccount>,
    
    /// Yield vault for reward distribution
    #[account(
        mut,
        seeds = [b"yield_vault", game_state.key().as_ref()],
        bump,
    )]
    pub yield_vault: Account<'info, TokenAccount>,
    
    /// CHECK: Token authority (PDA)
    #[account(
        seeds = [b"game_state"],
        bump = game_state.bump,
    )]
    pub token_authority: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ResolveRound>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let interrogation = &mut ctx.accounts.interrogation;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Validate game state
    game_state.validate_not_paused()?;
    
    // Check if can resolve
    require!(
        interrogation.can_resolve(now),
        SnitchError::PendingReveals
    );
    require!(
        interrogation.outcome == Outcome::Pending,
        SnitchError::AlreadyResolved
    );
    
    // Get choices (None if not revealed)
    let choice_a = interrogation.choice_a;
    let choice_b = interrogation.choice_b;
    
    // Determine outcome and calculate rewards
    let config = &game_state.config;
    let balance_a = ctx.accounts.player_a_token.amount;
    let balance_b = ctx.accounts.player_b_token.amount;
    
    // Calculate outcome and rewards
    let (
        outcome, 
        reward_a, 
        reward_b, 
        burn_a, 
        burn_b,
        penalty_a,
        penalty_b,
    ) = calculate_outcome(
        choice_a,
        choice_b,
        balance_a,
        balance_b,
        game_state.yield_pool,
        config,
    )?;
    
    // Update interrogation
    interrogation.outcome = outcome;
    interrogation.player_a_reward = reward_a;
    interrogation.player_b_reward = reward_b;
    interrogation.amount_burned = burn_a.saturating_add(burn_b);
    interrogation.phase = Phase::Resolved;
    interrogation.rewards_distributed = true;
    
    // Update game state
    game_state.active_interrogation = None;
    game_state.total_rounds_completed = game_state.total_rounds_completed
        .checked_add(1)
        .ok_or(SnitchError::Overflow)?;
    game_state.total_burned = game_state.total_burned
        .checked_add(interrogation.amount_burned)
        .ok_or(SnitchError::Overflow)?;
    
    // Update player profiles
    let player_a_profile = &mut ctx.accounts.player_a_profile;
    let player_b_profile = &mut ctx.accounts.player_b_profile;
    
    // Clear current round
    player_a_profile.current_round = 0;
    player_b_profile.current_round = 0;
    
    // Apply penalties to profiles
    if penalty_a > 0 {
        player_a_profile.add_penalty(penalty_a)?;
    }
    if penalty_b > 0 {
        player_b_profile.add_penalty(penalty_b)?;
    }
    
    // Add rewards to pending (or handle immediate transfer)
    if reward_a > 0 {
        player_a_profile.add_pending_rewards(reward_a)?;
    }
    if reward_b > 0 {
        player_b_profile.add_pending_rewards(reward_b)?;
    }
    
    // Execute token transfers and burns
    let seeds = &[
        b"game_state",
        &[game_state.bump],
    ];
    let signer = &[&seeds[..]];
    
    // Burn from player A if needed
    if burn_a > 0 {
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.player_a_token.to_account_info(),
                from: ctx.accounts.player_a_token.to_account_info(),
                authority: ctx.accounts.token_authority.to_account_info(),
            },
        );
        // Note: Burning from player requires delegate or direct transfer
        // In practice, we'd need player-signed burn or escrow
        // For this implementation, penalties are tracked and claimed from yield
    }
    
    // Distribute yield rewards from vault
    if reward_a > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.yield_vault.to_account_info(),
                to: ctx.accounts.player_a_token.to_account_info(),
                authority: ctx.accounts.token_authority.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, reward_a)?;
    }
    
    if reward_b > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.yield_vault.to_account_info(),
                to: ctx.accounts.player_b_token.to_account_info(),
                authority: ctx.accounts.token_authority.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, reward_b)?;
    }
    
    // Update yield pool
    game_state.yield_pool = game_state.yield_pool
        .saturating_sub(reward_a)
        .saturating_sub(reward_b);
    
    game_state.total_rewards_distributed = game_state.total_rewards_distributed
        .checked_add(reward_a)
        .ok_or(SnitchError::Overflow)?
        .checked_add(reward_b)
        .ok_or(SnitchError::Overflow)?;
    
    // Emit event
    emit!(RoundResolved {
        round_id: interrogation.round_id,
        player_a: interrogation.player_a,
        player_b: interrogation.player_b,
        choice_a: choice_a.unwrap_or(Choice::Cooperate),
        choice_b: choice_b.unwrap_or(Choice::Cooperate),
        outcome,
        player_a_reward: reward_a,
        player_b_reward: reward_b,
        amount_burned: interrogation.amount_burned,
        timestamp: now,
    });
    
    msg!("Round {} resolved: {:?}", interrogation.round_id, outcome);
    msg!("Player A reward: {}, Player B reward: {}", reward_a, reward_b);
    msg!("Total burned: {}", interrogation.amount_burned);
    
    Ok(())
}

/// Calculate the outcome and rewards
fn calculate_outcome(
    choice_a: Option<Choice>,
    choice_b: Option<Choice>,
    balance_a: u64,
    balance_b: u64,
    yield_pool: u64,
    config: &GameConfig,
) -> Result<(Outcome, u64, u64, u64, u64, u64, u64)> {
    // Default to timeout if no reveals
    let choice_a = choice_a.unwrap_or(Choice::Cooperate);
    let choice_b = choice_b.unwrap_or(Choice::Cooperate);
    
    let bps = config.bps_denominator as u64;
    
    match (choice_a, choice_b) {
        // Both cooperate: both get 5% yield
        (Choice::Cooperate, Choice::Cooperate) => {
            let yield_a = (balance_a as u128)
                .checked_mul(config.cooperation_yield_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            let yield_b = (balance_b as u128)
                .checked_mul(config.cooperation_yield_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            // Cap at available yield pool
            let actual_yield_a = yield_a.min(yield_pool / 2);
            let actual_yield_b = yield_b.min((yield_pool - actual_yield_a).max(0));
            
            Ok((
                Outcome::MutualCooperation,
                actual_yield_a,
                actual_yield_b,
                0, // burn_a
                0, // burn_b
                0, // penalty_a
                0, // penalty_b
            ))
        }
        
        // A snitches, B cooperates: A gets 50% of B's bag
        (Choice::Snitch, Choice::Cooperate) => {
            let reward_a = (balance_b as u128)
                .checked_mul(config.snitch_reward_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            let penalty_b = reward_a; // B loses what A gains
            
            Ok((
                Outcome::PlayerASnitched,
                reward_a,
                0,
                0,   // burn_a
                0,   // burn_b (penalty is transfer, not burn)
                0,   // penalty_a
                penalty_b,
            ))
        }
        
        // B snitches, A cooperates: B gets 50% of A's bag
        (Choice::Cooperate, Choice::Snitch) => {
            let reward_b = (balance_a as u128)
                .checked_mul(config.snitch_reward_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            let penalty_a = reward_b; // A loses what B gains
            
            Ok((
                Outcome::PlayerBSnitched,
                0,
                reward_b,
                0,   // burn_a (penalty is transfer, not burn)
                0,   // burn_b
                penalty_a,
                0,   // penalty_b
            ))
        }
        
        // Both snitch: both lose 25%, burned
        (Choice::Snitch, Choice::Snitch) => {
            let penalty_a = (balance_a as u128)
                .checked_mul(config.double_snitch_penalty_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            let penalty_b = (balance_b as u128)
                .checked_mul(config.double_snitch_penalty_bps as u128)
                .ok_or(SnitchError::Overflow)?
                .checked_div(bps as u128)
                .ok_or(SnitchError::Underflow)? as u64;
            
            Ok((
                Outcome::MutualSnitching,
                0,
                0,
                penalty_a, // burned
                penalty_b, // burned
                penalty_a,
                penalty_b,
            ))
        }
    }
}
