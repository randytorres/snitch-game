use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod utils;

pub use state::*;
pub use instructions::*;
pub use utils::*;

declare_id!("SN1tChXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod snitch {
    use super::*;

    /// Initialize the game state
    pub fn initialize(
        ctx: Context<Initialize>,
        config: GameConfig,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, config)
    }

    /// Select 2 random players for interrogation (admin or VRF)
    pub fn select_players(
        ctx: Context<SelectPlayers>,
        random_seed: [u8; 32],
    ) -> Result<()> {
        instructions::select_players::handler(ctx, random_seed)
    }

    /// Commit a choice (COOPERATE or SNITCH) using salted hash
    pub fn commit_choice(
        ctx: Context<CommitChoice>,
        commitment: [u8; 32],
    ) -> Result<()> {
        instructions::commit_choice::handler(ctx, commitment)
    }

    /// Reveal the committed choice with salt verification
    pub fn reveal_choice(
        ctx: Context<RevealChoice>,
        choice: u8,
        salt: [u8; 32],
    ) -> Result<()> {
        instructions::reveal_choice::handler(ctx, choice, salt)
    }

    /// Resolve the round after both reveals or timeout
    pub fn resolve_round(
        ctx: Context<ResolveRound>,
    ) -> Result<()> {
        instructions::resolve_round::handler(ctx)
    }

    /// Claim pending rewards
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        instructions::claim_rewards::handler(ctx)
    }

    /// Update game config (admin only)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        config: GameConfig,
    ) -> Result<()> {
        instructions::update_config::handler(ctx, config)
    }

    /// Emergency pause (admin only)
    pub fn pause(
        ctx: Context<AdminOnly>,
    ) -> Result<()> {
        instructions::admin::pause_handler(ctx)
    }

    /// Resume game (admin only)
    pub fn resume(
        ctx: Context<AdminOnly>,
    ) -> Result<()> {
        instructions::admin::resume_handler(ctx)
    }

    /// Update player cooldown manually (admin only, for emergencies)
    pub fn reset_cooldown(
        ctx: Context<ResetCooldown>,
        player: Pubkey,
    ) -> Result<()> {
        instructions::admin::reset_cooldown_handler(ctx, player)
    }
}

#[error_code]
pub enum SnitchError {
    #[msg("Game is currently paused")]
    GamePaused,
    #[msg("Invalid game configuration")]
    InvalidConfig,
    #[msg("Unauthorized caller")]
    Unauthorized,
    #[msg("No active interrogation round")]
    NoActiveRound,
    #[msg("Round is still active")]
    RoundActive,
    #[msg("Player already committed")]
    AlreadyCommitted,
    #[msg("Player not committed")]
    NotCommitted,
    #[msg("Invalid commitment")]
    InvalidCommitment,
    #[msg("Invalid choice - must be 0 (COOPERATE) or 1 (SNITCH)")]
    InvalidChoice,
    #[msg("Commit-reveal phase has ended")]
    CommitPhaseEnded,
    #[msg("Reveal phase has not started")]
    RevealPhaseNotStarted,
    #[msg("Reveal phase has ended")]
    RevealPhaseEnded,
    #[msg("Both players have not revealed yet")]
    PendingReveals,
    #[msg("Player already revealed")]
    AlreadyRevealed,
    #[msg("Round already resolved")]
    AlreadyResolved,
    #[msg("Player is on cooldown")]
    PlayerOnCooldown,
    #[msg("Player does not meet minimum balance requirement")]
    InsufficientBalance,
    #[msg("Player already selected this round")]
    PlayerAlreadySelected,
    #[msg("Player not in this interrogation")]
    PlayerNotInInterrogation,
    #[msg("No rewards to claim")]
    NoRewards,
    #[msg("Player profile not found")]
    ProfileNotFound,
    #[msg("Overflow in calculation")]
    Overflow,
    #[msg("Underflow in calculation")]
    Underflow,
    #[msg("Invalid outcome state")]
    InvalidOutcome,
    #[msg("Cannot select from empty holder pool")]
    EmptyHolderPool,
    #[msg("Self-selection not allowed")]
    SelfSelection,
    #[msg("Invalid random seed")]
    InvalidSeed,
    #[msg("Token transfer failed")]
    TransferFailed,
    #[msg("Token burn failed")]
    BurnFailed,
}

#[event]
pub struct RoundStarted {
    pub round_id: u64,
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ChoiceCommitted {
    pub round_id: u64,
    pub player: Pubkey,
    pub commitment: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct ChoiceRevealed {
    pub round_id: u64,
    pub player: Pubkey,
    pub choice: Choice,
    pub timestamp: i64,
}

#[event]
pub struct RoundResolved {
    pub round_id: u64,
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub choice_a: Choice,
    pub choice_b: Choice,
    pub outcome: Outcome,
    pub player_a_reward: u64,
    pub player_b_reward: u64,
    pub amount_burned: u64,
    pub timestamp: i64,
}

#[event]
pub struct RewardsClaimed {
    pub player: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PlayerSelected {
    pub player: Pubkey,
    pub round_id: u64,
    pub timestamp: i64,
}
