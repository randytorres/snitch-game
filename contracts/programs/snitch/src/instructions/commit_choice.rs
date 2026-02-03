use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use solana_program::hash::hashv;

use crate::state::*;
use crate::SnitchError;
use crate::ChoiceCommitted;

#[derive(Accounts)]
#[instruction(commitment: [u8; 32])]
pub struct CommitChoice<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(
        seeds = [b"game_state"],
        bump = game_state.bump,
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        mut,
        constraint = interrogation.round_id == player_profile.current_round @ SnitchError::PlayerNotInInterrogation,
    )]
    pub interrogation: Account<'info, Interrogation>,
    
    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump = player_profile.bump,
        constraint = player_profile.player == player.key() @ SnitchError::Unauthorized,
    )]
    pub player_profile: Account<'info, PlayerProfile>,
}

pub fn handler(
    ctx: Context<CommitChoice>,
    commitment: [u8; 32],
) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    let interrogation = &mut ctx.accounts.interrogation;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Validate game state
    game_state.validate_not_paused()?;
    
    // Check phase
    let current_phase = interrogation.get_current_phase(now);
    require!(
        current_phase == Phase::Commit,
        SnitchError::CommitPhaseEnded
    );
    require!(
        interrogation.can_commit(now),
        SnitchError::CommitPhaseEnded
    );
    
    // Validate player is in this round
    let player_key = ctx.accounts.player.key();
    require!(
        interrogation.is_player_in_round(&player_key),
        SnitchError::PlayerNotInInterrogation
    );
    
    // Store commitment based on player
    let is_player_a = interrogation.player_a == player_key;
    
    if is_player_a {
        require!(
            interrogation.commitment_a.is_none(),
            SnitchError::AlreadyCommitted
        );
        interrogation.commitment_a = Some(commitment);
    } else {
        require!(
            interrogation.commitment_b.is_none(),
            SnitchError::AlreadyCommitted
        );
        interrogation.commitment_b = Some(commitment);
    }
    
    // Update phase if both committed
    if interrogation.both_committed() {
        // Optionally move to reveal phase early if both committed
        msg!("Both players have committed!");
    }
    
    // Emit event
    emit!(ChoiceCommitted {
        round_id: interrogation.round_id,
        player: player_key,
        commitment,
        timestamp: now,
    });
    
    msg!("Player {} committed in round {}", player_key, interrogation.round_id);
    
    Ok(())
}

/// Verify a commitment against choice and salt
pub fn verify_commitment(
    commitment: [u8; 32],
    choice: u8,
    salt: [u8; 32],
    player: &Pubkey,
    round_id: u64,
) -> bool {
    let round_bytes = round_id.to_le_bytes();
    let hash_input = [
        player.as_ref(),
        &[choice],
        &salt,
        &round_bytes,
    ];
    
    let computed_hash = hashv(&hash_input.iter().map(|b| *b).collect::<Vec<_>>()
    );
    
    commitment == computed_hash.to_bytes()
}
