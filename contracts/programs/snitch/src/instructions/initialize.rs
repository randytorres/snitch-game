use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};
use solana_program::clock::Clock;

use crate::state::*;
use crate::SnitchError;
use crate::{RoundStarted, PlayerSelected};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: This will be the game authority
    pub authority: AccountInfo<'info>,
    
    #[account(
        init,
        payer = payer,
        space = GameState::SIZE,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    /// The SNITCH token mint
    pub token_mint: Account<'info, Mint>,
    
    /// CHECK: Yield vault PDA (token account)
    #[account(
        init,
        payer = payer,
        seeds = [b"yield_vault", game_state.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = game_state,
    )]
    pub yield_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Initialize>,
    config: GameConfig,
) -> Result<()> {
    // Validate configuration
    config.validate()?;
    
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;
    
    game_state.authority = ctx.accounts.authority.key();
    game_state.token_mint = ctx.accounts.token_mint.key();
    game_state.current_round = 0;
    game_state.yield_pool = 0;
    game_state.config = config;
    game_state.paused = false;
    game_state.active_interrogation = None;
    game_state.total_rounds_completed = 0;
    game_state.total_burned = 0;
    game_state.total_rewards_distributed = 0;
    game_state.bump = ctx.bumps.game_state;
    game_state.reserved = [0; 32];
    
    msg!("Game initialized at {}", clock.unix_timestamp);
    msg!("Authority: {}", game_state.authority);
    msg!("Token mint: {}", game_state.token_mint);
    
    Ok(())
}

/// Accounts for selecting players
#[derive(Accounts)]
#[instruction(random_seed: [u8; 32])]
pub struct SelectPlayers<'info> {
    #[account(mut)]
    pub selector: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,
    
    /// CHECK: Player A's token account (for balance check)
    #[account(
        constraint = player_a_token.mint == game_state.token_mint @ SnitchError::Unauthorized,
    )]
    pub player_a_token: Account<'info, TokenAccount>,
    
    /// CHECK: Player B's token account (for balance check)
    #[account(
        constraint = player_b_token.mint == game_state.token_mint @ SnitchError::Unauthorized,
        constraint = player_b_token.owner != player_a_token.owner @ SnitchError::SelfSelection,
    )]
    pub player_b_token: Account<'info, TokenAccount>,
    
    /// Player A profile (init if needed)
    #[account(
        init_if_needed,
        payer = selector,
        space = PlayerProfile::SIZE,
        seeds = [b"player", player_a_token.owner.as_ref()],
        bump,
    )]
    pub player_a_profile: Account<'info, PlayerProfile>,
    
    /// Player B profile (init if needed)
    #[account(
        init_if_needed,
        payer = selector,
        space = PlayerProfile::SIZE,
        seeds = [b"player", player_b_token.owner.as_ref()],
        bump,
    )]
    pub player_b_profile: Account<'info, PlayerProfile>,
    
    /// New interrogation round account
    #[account(
        init,
        payer = selector,
        space = Interrogation::SIZE,
        seeds = [
            b"interrogation",
            game_state.key().as_ref(),
            (game_state.current_round + 1).to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub interrogation: Account<'info, Interrogation>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<SelectPlayers>,
    random_seed: [u8; 32],
) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Validate game state
    game_state.validate_not_paused()?;
    game_state.validate_vrf_oracle(&ctx.accounts.selector)?;
    
    // Check no active interrogation
    require!(
        game_state.active_interrogation.is_none(),
        SnitchError::RoundActive
    );
    
    // Validate minimum balance for both players
    let config = &game_state.config;
    require!(
        ctx.accounts.player_a_token.amount >= config.min_balance_threshold,
        SnitchError::InsufficientBalance
    );
    require!(
        ctx.accounts.player_b_token.amount >= config.min_balance_threshold,
        SnitchError::InsufficientBalance
    );
    
    // Get player addresses
    let player_a = ctx.accounts.player_a_token.owner;
    let player_b = ctx.accounts.player_b_token.owner;
    
    // Check cooldowns
    require!(
        !ctx.accounts.player_a_profile.is_on_cooldown(now),
        SnitchError::PlayerOnCooldown
    );
    require!(
        !ctx.accounts.player_b_profile.is_on_cooldown(now),
        SnitchError::PlayerOnCooldown
    );
    
    // Validate players aren't already in a round
    require!(
        ctx.accounts.player_a_profile.current_round == 0,
        SnitchError::PlayerAlreadySelected
    );
    require!(
        ctx.accounts.player_b_profile.current_round == 0,
        SnitchError::PlayerAlreadySelected
    );
    
    // Increment round
    game_state.current_round = game_state.current_round
        .checked_add(1)
        .ok_or(SnitchError::Overflow)?;
    
    let round_id = game_state.current_round;
    
    // Initialize interrogation
    let interrogation = &mut ctx.accounts.interrogation;
    interrogation.round_id = round_id;
    interrogation.player_a = player_a;
    interrogation.player_b = player_b;
    interrogation.commitment_a = None;
    interrogation.commitment_b = None;
    interrogation.choice_a = None;
    interrogation.choice_b = None;
    interrogation.revealed_a = false;
    interrogation.revealed_b = false;
    interrogation.phase = Phase::Commit;
    interrogation.started_at = now;
    interrogation.commit_ends_at = now + config.commit_duration;
    interrogation.reveal_ends_at = now + config.commit_duration + config.reveal_duration;
    interrogation.outcome = Outcome::Pending;
    interrogation.player_a_reward = 0;
    interrogation.player_b_reward = 0;
    interrogation.amount_burned = 0;
    interrogation.rewards_distributed = false;
    interrogation.bump = ctx.bumps.interrogation;
    
    // Update game state
    game_state.active_interrogation = Some(interrogation.key());
    
    // Update player profiles
    let player_a_profile = &mut ctx.accounts.player_a_profile;
    player_a_profile.player = player_a;
    player_a_profile.times_selected = player_a_profile.times_selected
        .checked_add(1)
        .ok_or(SnitchError::Overflow)?;
    player_a_profile.current_round = round_id;
    player_a_profile.set_cooldown(config.cooldown_period, now);
    player_a_profile.bump = ctx.bumps.player_a_profile;
    
    let player_b_profile = &mut ctx.accounts.player_b_profile;
    player_b_profile.player = player_b;
    player_b_profile.times_selected = player_b_profile.times_selected
        .checked_add(1)
        .ok_or(SnitchError::Overflow)?;
    player_b_profile.current_round = round_id;
    player_b_profile.set_cooldown(config.cooldown_period, now);
    player_b_profile.bump = ctx.bumps.player_b_profile;
    
    // Emit events
    emit!(RoundStarted {
        round_id,
        player_a,
        player_b,
        timestamp: now,
    });
    
    emit!(PlayerSelected {
        player: player_a,
        round_id,
        timestamp: now,
    });
    
    emit!(PlayerSelected {
        player: player_b,
        round_id,
        timestamp: now,
    });
    
    msg!("Round {} started: {} vs {}", round_id, player_a, player_b);
    msg!("Commit phase ends at: {}", interrogation.commit_ends_at);
    
    Ok(())
}
