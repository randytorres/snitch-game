use anchor_lang::prelude::*;
use solana_program::clock::Clock;

use crate::state::*;
use crate::SnitchError;

// ==================== Admin Instructions ====================

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump,
        constraint = game_state.authority == admin.key() @ SnitchError::Unauthorized,
    )]
    pub game_state: Account<'info, GameState>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump,
        constraint = game_state.authority == admin.key() @ SnitchError::Unauthorized,
    )]
    pub game_state: Account<'info, GameState>,
}

pub fn update_config_handler(
    ctx: Context<UpdateConfig>,
    config: GameConfig,
) -> Result<()> {
    // Validate new configuration
    config.validate()?;
    
    let game_state = &mut ctx.accounts.game_state;
    game_state.config = config;
    
    msg!("Game configuration updated by admin");
    
    Ok(())
}

pub fn pause_handler(ctx: Context<AdminOnly>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    
    require!(!game_state.paused, SnitchError::InvalidConfig);
    
    game_state.paused = true;
    
    msg!("Game paused by admin {} at {}", 
        ctx.accounts.admin.key(),
        Clock::get()?.unix_timestamp
    );
    
    Ok(())
}

pub fn resume_handler(ctx: Context<AdminOnly>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    
    require!(game_state.paused, SnitchError::InvalidConfig);
    
    game_state.paused = false;
    
    msg!("Game resumed by admin {} at {}", 
        ctx.accounts.admin.key(),
        Clock::get()?.unix_timestamp
    );
    
    Ok(())
}

#[derive(Accounts)]
pub struct ResetCooldown<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"game_state"],
        bump = game_state.bump,
        constraint = game_state.authority == admin.key() @ SnitchError::Unauthorized,
    )]
    pub game_state: Account<'info, GameState>,
    
    /// CHECK: Player profile to reset
    #[account(
        mut,
        seeds = [b"player", player.as_ref()],
        bump,
    )]
    pub player_profile: Account<'info, PlayerProfile>,
}

pub fn reset_cooldown_handler(
    _ctx: Context<ResetCooldown>,
    player: Pubkey,
) -> Result<()> {
    let player_profile = &mut _ctx.accounts.player_profile;
    
    player_profile.clear_cooldown();
    
    msg!("Cooldown reset for player {} by admin {}", 
        player,
        _ctx.accounts.admin.key()
    );
    
    Ok(())
}
