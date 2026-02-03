use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use solana_program::hash::hashv;

use crate::state::*;
use crate::SnitchError;
use crate::ChoiceRevealed;

#[derive(Accounts)]
#[instruction(choice: u8, salt: [u8; 32])]
pub struct RevealChoice<'info> {
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
    ctx: Context<RevealChoice>,
    choice: u8,
    salt: [u8; 32],
) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    let interrogation = &mut ctx.accounts.interrogation;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Validate game state
    game_state.validate_not_paused()?;
    
    // Validate choice
    let choice_enum = Choice::from_u8(choice)?;
    
    // Check phase
    let current_phase = interrogation.get_current_phase(now);
    require!(
        current_phase == Phase::Reveal,
        if now <= interrogation.commit_ends_at {
            SnitchError::RevealPhaseNotStarted
        } else {
            SnitchError::RevealPhaseEnded
        }
    );
    require!(
        interrogation.can_reveal(now),
        SnitchError::RevealPhaseEnded
    );
    
    // Validate player is in this round
    let player_key = ctx.accounts.player.key();
    require!(
        interrogation.is_player_in_round(&player_key),
        SnitchError::PlayerNotInInterrogation
    );
    
    // Get stored commitment
    let is_player_a = interrogation.player_a == player_key;
    
    let stored_commitment = if is_player_a {
        require!(
            !interrogation.revealed_a,
            SnitchError::AlreadyRevealed
        );
        interrogation.commitment_a.ok_or(SnitchError::NotCommitted)?
    } else {
        require!(
            !interrogation.revealed_b,
            SnitchError::AlreadyRevealed
        );
        interrogation.commitment_b.ok_or(SnitchError::NotCommitted)?
    };
    
    // Verify commitment
    let valid = verify_commitment(
        stored_commitment,
        choice,
        salt,
        &player_key,
        interrogation.round_id,
    );
    require!(valid, SnitchError::InvalidCommitment);
    
    // Store revealed choice
    if is_player_a {
        interrogation.choice_a = Some(choice_enum);
        interrogation.revealed_a = true;
    } else {
        interrogation.choice_b = Some(choice_enum);
        interrogation.revealed_b = true;
    }
    
    // Update player stats
    let player_profile = &mut ctx.accounts.player_profile;
    match choice_enum {
        Choice::Cooperate => {
            player_profile.times_cooperated = player_profile.times_cooperated
                .checked_add(1)
                .ok_or(SnitchError::Overflow)?;
        }
        Choice::Snitch => {
            player_profile.times_snitched = player_profile.times_snitched
                .checked_add(1)
                .ok_or(SnitchError::Overflow)?;
        }
    }
    
    // Update phase if both revealed
    if interrogation.both_revealed() {
        interrogation.phase = Phase::Resolved;
    }
    
    // Emit event
    emit!(ChoiceRevealed {
        round_id: interrogation.round_id,
        player: player_key,
        choice: choice_enum,
        timestamp: now,
    });
    
    msg!("Player {} revealed {:?} in round {}", 
        player_key, choice_enum, interrogation.round_id);
    
    Ok(())
}

/// Verify a commitment against choice and salt
fn verify_commitment(
    commitment: [u8; 32],
    choice: u8,
    salt: [u8; 32],
    player: &Pubkey,
    round_id: u64,
) -> bool {
    let round_bytes = round_id.to_le_bytes();
    
    // Hash: H(player_pubkey || choice || salt || round_id)
    let hash_result = hashv(&[
        player.as_ref(),
        &[choice],
        &salt,
        &round_bytes,
    ]);
    
    let computed_hash = hash_result.to_bytes();
    commitment == computed_hash
}
