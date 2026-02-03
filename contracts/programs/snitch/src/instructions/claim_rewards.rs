use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use solana_program::clock::Clock;

use crate::state::*;
use crate::SnitchError;
use crate::RewardsClaimed;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(
        seeds = [b"game_state"],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump = player_profile.bump,
        constraint = player_profile.player == player.key() @ SnitchError::Unauthorized,
    )]
    pub player_profile: Account<'info, PlayerProfile>,
    
    /// Player's token account
    #[account(
        mut,
        constraint = player_token.owner == player.key() @ SnitchError::Unauthorized,
        constraint = player_token.mint == game_state.token_mint @ SnitchError::Unauthorized,
    )]
    pub player_token: Account<'info, TokenAccount>,
    
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

pub fn handler(ctx: Context<ClaimRewards>) -> Result<()> {
    let player_profile = &mut ctx.accounts.player_profile;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Check for pending rewards
    let amount = player_profile.pending_rewards;
    require!(amount > 0, SnitchError::NoRewards);
    
    // Verify yield vault has enough
    require!(
        ctx.accounts.yield_vault.amount >= amount,
        SnitchError::InsufficientBalance
    );
    
    // Clear pending rewards before transfer (reentrancy protection)
    player_profile.pending_rewards = 0;
    
    // Transfer rewards from yield vault
    let seeds = &[
        b"game_state",
        &[ctx.accounts.game_state.bump],
    ];
    let signer = &[&seeds[..]];
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.yield_vault.to_account_info(),
            to: ctx.accounts.player_token.to_account_info(),
            authority: ctx.accounts.token_authority.to_account_info(),
        },
        signer,
    );
    
    token::transfer(transfer_ctx, amount)?;
    
    // Update yield pool
    let game_state = &mut ctx.accounts.game_state;
    game_state.yield_pool = game_state.yield_pool
        .checked_sub(amount)
        .ok_or(SnitchError::Underflow)?;
    game_state.total_rewards_distributed = game_state.total_rewards_distributed
        .checked_add(amount)
        .ok_or(SnitchError::Overflow)?;
    
    // Emit event
    emit!(RewardsClaimed {
        player: ctx.accounts.player.key(),
        amount,
        timestamp: now,
    });
    
    msg!("Player {} claimed {} rewards", ctx.accounts.player.key(), amount);
    
    Ok(())
}
