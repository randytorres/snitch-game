use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

/// Game configuration parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct GameConfig {
    /// Minimum token balance to be eligible for selection (0.1% of supply = 1/1000)
    pub min_balance_threshold: u64,
    /// Cooldown period in seconds (7 days = 604800 seconds)
    pub cooldown_period: i64,
    /// Commit phase duration in seconds (12 hours = 43200 seconds)
    pub commit_duration: i64,
    /// Reveal phase duration in seconds (12 hours = 43200 seconds)
    pub reveal_duration: i64,
    /// Yield percentage for mutual cooperation (5% = 500 basis points)
    pub cooperation_yield_bps: u16,
    /// Percentage snitch takes from victim (50% = 5000 basis points)
    pub snitch_reward_bps: u16,
    /// Percentage both lose when both snitch (25% = 2500 basis points)
    pub double_snitch_penalty_bps: u16,
    /// Transfer tax in basis points (2% = 200 bps)
    pub transfer_tax_bps: u16,
    /// Basis points denominator (10000 = 100%)
    pub bps_denominator: u16,
    /// Enable VRF for selection (false = admin/manual selection)
    pub use_vrf: bool,
    /// Authorized VRF oracle (if use_vrf is true)
    pub vrf_oracle: Option<Pubkey>,
}

impl GameConfig {
    pub fn validate(&self) -> Result<()> {
        require!(self.bps_denominator == 10000, SnitchError::InvalidConfig);
        require!(self.cooperation_yield_bps <= 10000, SnitchError::InvalidConfig);
        require!(self.snitch_reward_bps <= 10000, SnitchError::InvalidConfig);
        require!(self.double_snitch_penalty_bps <= 10000, SnitchError::InvalidConfig);
        require!(self.transfer_tax_bps <= 1000, SnitchError::InvalidConfig); // Max 10%
        require!(self.commit_duration > 0, SnitchError::InvalidConfig);
        require!(self.reveal_duration > 0, SnitchError::InvalidConfig);
        require!(self.cooldown_period > 0, SnitchError::InvalidConfig);
        Ok(())
    }

    pub fn default_with_mint(mint: &Mint) -> Self {
        let total_supply = mint.supply;
        // 0.1% of total supply
        let min_balance = total_supply / 1000;
        
        Self {
            min_balance_threshold: min_balance.max(1_000_000), // At least 1 token with 6 decimals
            cooldown_period: 604800, // 7 days
            commit_duration: 43200,   // 12 hours
            reveal_duration: 43200,   // 12 hours
            cooperation_yield_bps: 500,   // 5%
            snitch_reward_bps: 5000,      // 50%
            double_snitch_penalty_bps: 2500, // 25%
            transfer_tax_bps: 200,        // 2%
            bps_denominator: 10000,
            use_vrf: false,
            vrf_oracle: None,
        }
    }
}

/// Global game state PDA
#[account]
pub struct GameState {
    /// Authority/admin of the game
    pub authority: Pubkey,
    /// The SNITCH token mint
    pub token_mint: Pubkey,
    /// Current round number
    pub current_round: u64,
    /// Total yield pool accumulated
    pub yield_pool: u64,
    /// Game configuration
    pub config: GameConfig,
    /// Whether the game is paused
    pub paused: bool,
    /// Current active interrogation (if any)
    pub active_interrogation: Option<Pubkey>,
    /// Total rounds completed
    pub total_rounds_completed: u64,
    /// Total tokens burned
    pub total_burned: u64,
    /// Total tokens distributed as rewards
    pub total_rewards_distributed: u64,
    /// Bump for PDA
    pub bump: u8,
    /// Reserved space for future upgrades
    pub reserved: [u8; 32],
}

impl GameState {
    pub const SIZE: usize = 
        8 +  // discriminator
        32 + // authority
        32 + // token_mint
        8 +  // current_round
        8 +  // yield_pool
        GameConfig::SIZE + // config
        1 +  // paused
        33 + // active_interrogation (Option<Pubkey>)
        8 +  // total_rounds_completed
        8 +  // total_burned
        8 +  // total_rewards_distributed
        1 +  // bump
        32;  // reserved

    pub fn validate_authority(&self, signer: &Signer) -> Result<()> {
        require!(
            self.authority == signer.key(),
            SnitchError::Unauthorized
        );
        Ok(())
    }

    pub fn validate_not_paused(&self) -> Result<()> {
        require!(!self.paused, SnitchError::GamePaused);
        Ok(())
    }

    pub fn validate_vrf_oracle(&self, signer: &Signer) -> Result<()> {
        if self.config.use_vrf {
            let oracle = self.config.vrf_oracle
                .ok_or(SnitchError::Unauthorized)?;
            require!(
                oracle == signer.key() || self.authority == signer.key(),
                SnitchError::Unauthorized
            );
        } else {
            self.validate_authority(signer)?;
        }
        Ok(())
    }
}

/// Choice enum for player decisions
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Cooperate = 0,
    Snitch = 1,
}

impl Choice {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Choice::Cooperate),
            1 => Ok(Choice::Snitch),
            _ => Err(SnitchError::InvalidChoice.into()),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Choice::Cooperate => 0,
            Choice::Snitch => 1,
        }
    }
}

/// Outcome of an interrogation round
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Pending = 0,
    MutualCooperation = 1,
    PlayerASnitched = 2,
    PlayerBSnitched = 3,
    MutualSnitching = 4,
    Timeout = 5,
}

/// Phase of an interrogation round
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Commit = 0,
    Reveal = 1,
    Resolved = 2,
}

/// Interrogation round PDA
#[account]
pub struct Interrogation {
    /// Unique round ID
    pub round_id: u64,
    /// Player A pubkey
    pub player_a: Pubkey,
    /// Player B pubkey
    pub player_b: Pubkey,
    /// Player A's commitment hash
    pub commitment_a: Option<[u8; 32]>,
    /// Player B's commitment hash
    pub commitment_b: Option<[u8; 32]>,
    /// Player A's revealed choice
    pub choice_a: Option<Choice>,
    /// Player B's revealed choice
    pub choice_b: Option<Choice>,
    /// Whether player A has revealed
    pub revealed_a: bool,
    /// Whether player B has revealed
    pub revealed_b: bool,
    /// Current phase
    pub phase: Phase,
    /// Round start timestamp
    pub started_at: i64,
    /// Commit phase ends at
    pub commit_ends_at: i64,
    /// Reveal phase ends at
    pub reveal_ends_at: i64,
    /// Final outcome
    pub outcome: Outcome,
    /// Rewards for player A (if positive) or penalty (calculated)
    pub player_a_reward: u64,
    /// Rewards for player B (if positive) or penalty (calculated)
    pub player_b_reward: u64,
    /// Amount burned in this round
    pub amount_burned: u64,
    /// Whether rewards have been distributed
    pub rewards_distributed: bool,
    /// Bump for PDA
    pub bump: u8,
}

impl Interrogation {
    pub const SIZE: usize = 
        8 +   // discriminator
        8 +   // round_id
        32 +  // player_a
        32 +  // player_b
        33 +  // commitment_a (Option<[u8; 32]>)
        33 +  // commitment_b (Option<[u8; 32]>)
        2 +   // choice_a (Option<Choice>)
        2 +   // choice_b (Option<Choice>)
        1 +   // revealed_a
        1 +   // revealed_b
        1 +   // phase
        8 +   // started_at
        8 +   // commit_ends_at
        8 +   // reveal_ends_at
        1 +   // outcome
        8 +   // player_a_reward
        8 +   // player_b_reward
        8 +   // amount_burned
        1 +   // rewards_distributed
        1;    // bump

    pub fn get_current_phase(&self, now: i64) -> Phase {
        if self.outcome != Outcome::Pending {
            return Phase::Resolved;
        }
        if now > self.reveal_ends_at {
            return Phase::Resolved;
        }
        if now > self.commit_ends_at {
            return Phase::Reveal;
        }
        Phase::Commit
    }

    pub fn is_player_in_round(&self, player: &Pubkey) -> bool {
        self.player_a == *player || self.player_b == *player
    }

    pub fn get_player_index(&self, player: &Pubkey) -> Result<usize> {
        if self.player_a == *player {
            Ok(0)
        } else if self.player_b == *player {
            Ok(1)
        } else {
            Err(SnitchError::PlayerNotInInterrogation.into())
        }
    }

    pub fn both_committed(&self) -> bool {
        self.commitment_a.is_some() && self.commitment_b.is_some()
    }

    pub fn both_revealed(&self) -> bool {
        self.revealed_a && self.revealed_b
    }

    pub fn can_commit(&self, now: i64) -> bool {
        self.phase == Phase::Commit && now <= self.commit_ends_at
    }

    pub fn can_reveal(&self, now: i64) -> bool {
        (self.phase == Phase::Reveal || now > self.commit_ends_at) && now <= self.reveal_ends_at
    }

    pub fn can_resolve(&self, now: i64) -> bool {
        if self.outcome != Outcome::Pending {
            return false;
        }
        // Can resolve if both revealed or if reveal phase ended
        self.both_revealed() || now > self.reveal_ends_at
    }
}

/// Player profile PDA
#[account]
pub struct PlayerProfile {
    /// Player's wallet pubkey
    pub player: Pubkey,
    /// Total times selected for interrogation
    pub times_selected: u64,
    /// Total times cooperated
    pub times_cooperated: u64,
    /// Total times snitched
    pub times_snitched: u64,
    /// Total rewards earned
    pub total_rewards_earned: u64,
    /// Total penalties paid
    pub total_penalties_paid: u64,
    /// Pending rewards to claim
    pub pending_rewards: u64,
    /// Last selection timestamp
    pub last_selected_at: i64,
    /// Cooldown ends at
    pub cooldown_ends_at: i64,
    /// Whether player is currently on cooldown
    pub on_cooldown: bool,
    /// Current round if selected (0 if not)
    pub current_round: u64,
    /// Bump for PDA
    pub bump: u8,
    /// Reserved for future
    pub reserved: [u8; 16],
}

impl PlayerProfile {
    pub const SIZE: usize = 
        8 +  // discriminator
        32 + // player
        8 +  // times_selected
        8 +  // times_cooperated
        8 +  // times_snitched
        8 +  // total_rewards_earned
        8 +  // total_penalties_paid
        8 +  // pending_rewards
        8 +  // last_selected_at
        8 +  // cooldown_ends_at
        1 +  // on_cooldown
        8 +  // current_round
        1 +  // bump
        16;  // reserved

    pub fn is_on_cooldown(&self, now: i64) -> bool {
        self.on_cooldown && now < self.cooldown_ends_at
    }

    pub fn set_cooldown(&mut self, duration: i64, now: i64) {
        self.on_cooldown = true;
        self.cooldown_ends_at = now + duration;
        self.last_selected_at = now;
    }

    pub fn clear_cooldown(&mut self) {
        self.on_cooldown = false;
        self.cooldown_ends_at = 0;
    }

    pub fn add_pending_rewards(&mut self, amount: u64) -> Result<()> {
        self.pending_rewards = self.pending_rewards
            .checked_add(amount)
            .ok_or(SnitchError::Overflow)?;
        self.total_rewards_earned = self.total_rewards_earned
            .checked_add(amount)
            .ok_or(SnitchError::Overflow)?;
        Ok(())
    }

    pub fn add_penalty(&mut self, amount: u64) -> Result<()> {
        self.total_penalties_paid = self.total_penalties_paid
            .checked_add(amount)
            .ok_or(SnitchError::Overflow)?;
        Ok(())
    }

    pub fn claim_pending_rewards(&mut self) -> u64 {
        let amount = self.pending_rewards;
        self.pending_rewards = 0;
        amount
    }
}

/// Yield vault to hold accumulated tax
#[account]
pub struct YieldVault {
    /// Authority (GameState PDA)
    pub authority: Pubkey,
    /// Token mint
    pub token_mint: Pubkey,
    /// Total accumulated
    pub total_accumulated: u64,
    /// Bump for PDA
    pub bump: u8,
}

impl YieldVault {
    pub const SIZE: usize = 
        8 +  // discriminator
        32 + // authority
        32 + // token_mint
        8 +  // total_accumulated
        1;   // bump
}

impl GameConfig {
    pub const SIZE: usize = 
        8 +   // min_balance_threshold
        8 +   // cooldown_period
        8 +   // commit_duration
        8 +   // reveal_duration
        2 +   // cooperation_yield_bps
        2 +   // snitch_reward_bps
        2 +   // double_snitch_penalty_bps
        2 +   // transfer_tax_bps
        2 +   // bps_denominator
        1 +   // use_vrf
        33;   // vrf_oracle (Option<Pubkey>)
}

use crate::SnitchError;
