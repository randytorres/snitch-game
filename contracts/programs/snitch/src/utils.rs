use anchor_lang::prelude::*;

use crate::state::*;

/// Custom transfer hook for 2% tax to yield pool
/// This would be integrated with the token-2022 transfer hook
/// For token-2022 implementation, see: https://spl.solana.com/token-2022/extensions#transfer-hook
pub fn calculate_transfer_tax(
    amount: u64,
    config: &GameConfig,
) -> Result<u64> {
    let tax = (amount as u128)
        .checked_mul(config.transfer_tax_bps as u128)
        .ok_or(SnitchError::Overflow)?
        .checked_div(config.bps_denominator as u128)
        .ok_or(SnitchError::Underflow)? as u64;
    
    Ok(tax)
}

/// Calculate yield reward based on balance and cooperation
pub fn calculate_yield_reward(
    balance: u64,
    yield_bps: u16,
    bps_denominator: u16,
) -> Result<u64> {
    let reward = (balance as u128)
        .checked_mul(yield_bps as u128)
        .ok_or(SnitchError::Overflow)?
        .checked_div(bps_denominator as u128)
        .ok_or(SnitchError::Underflow)? as u64;
    
    Ok(reward)
}

use crate::SnitchError;

/// VRF verification placeholder
/// In production, integrate with Switchboard or Chainlink VRF
pub fn verify_vrf_proof(
    _proof: &[u8],
    _seed: &[u8; 32],
) -> Result<[u8; 32]> {
    // This is a placeholder - actual VRF verification would:
    // 1. Parse the VRF proof
    // 2. Verify the proof against the public key
    // 3. Return the random output
    
    // For production, use:
    // - Switchboard VRF: https://docs.switchboard.xyz/documentation/vrf
    // - Chainlink VRF: https://docs.chain.link/chainlink-vrf
    
    Ok([0u8; 32])
}

/// Select random players using verified randomness
pub fn select_random_players(
    random_seed: [u8; 32],
    eligible_players: &[Pubkey],
) -> Result<(Pubkey, Pubkey)> {
    require!(
        eligible_players.len() >= 2,
        SnitchError::EmptyHolderPool
    );
    
    // Use random seed to select first player
    let index_a = (u64::from_le_bytes([
        random_seed[0], random_seed[1], random_seed[2], random_seed[3],
        random_seed[4], random_seed[5], random_seed[6], random_seed[7],
    ]) % eligible_players.len() as u64) as usize;
    
    // Use different part of seed for second player
    let index_b = (u64::from_le_bytes([
        random_seed[8], random_seed[9], random_seed[10], random_seed[11],
        random_seed[12], random_seed[13], random_seed[14], random_seed[15],
    ]) % eligible_players.len() as u64) as usize;
    
    // Ensure different players
    let index_b = if index_a == index_b {
        (index_b + 1) % eligible_players.len()
    } else {
        index_b
    };
    
    Ok((eligible_players[index_a], eligible_players[index_b]))
}

/// Generate commitment hash for commit-reveal
pub fn generate_commitment(
    player: &Pubkey,
    choice: Choice,
    salt: [u8; 32],
    round_id: u64,
) -> [u8; 32] {
    use solana_program::hash::hashv;
    
    let round_bytes = round_id.to_le_bytes();
    let choice_byte = choice.to_u8();
    
    let hash_result = hashv(&[
        player.as_ref(),
        &[choice_byte],
        &salt,
        &round_bytes,
    ]);
    
    hash_result.to_bytes()
}
