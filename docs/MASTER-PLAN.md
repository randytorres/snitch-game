# $SNITCH — Complete Project Specification
### Prisoner's Dilemma Tokenomics on Solana

---

## 1. TECHNICAL ARCHITECTURE

### Smart Contract Structure (Solana/Anchor)

```
programs/
├── snitch-token/           # SPL Token with extended metadata
│   └── lib.rs
├── snitch-game/            # Core game logic
│   ├── state.rs            # PDAs: GameState, Interrogation, PlayerCommit
│   ├── instructions/
│   │   ├── initialize.rs
│   │   ├── select_players.rs
│   │   ├── commit_choice.rs
│   │   ├── reveal_choice.rs
│   │   ├── resolve_round.rs
│   │   └── claim_rewards.rs
│   └── lib.rs
└── snitch-treasury/        # Yield pool & burn mechanics
    └── lib.rs
```

**Key PDAs:**
- `GameState` — Global config (current round, last selection timestamp, yield pool balance)
- `Interrogation` — Per-round data (player1, player2, commits, reveals, status, outcome)
- `PlayerProfile` — Historical stats per wallet (games played, snitches, cooperates, net P/L)

### Random Selection Mechanism

**Primary: Switchboard VRF v2**
- Request randomness 1 hour before selection
- Use VRF result to seed deterministic selection from holder snapshot
- Fallback: Chainlink VRF (more expensive, same security)

### Commit-Reveal Scheme

**Phase 1: Commit (0-12 hours after selection)**
- Player submits: hash = keccak256("COOPERATE" || random_salt || player_pubkey)

**Phase 2: Reveal (12-24 hours after selection)**
- Contract verifies: keccak256(choice || salt || msg.sender) == stored_hash

**Timeout Handling:**
- No commit = auto-COOPERATE (punishes ghosting with vulnerability)
- No reveal = commitment burned, treated as COOPERATE
- Both timeout = round voided, new selection

### Yield/Burn Mechanics

**Outcomes:**
- Both cooperate → both get 5% yield from pool
- One snitches → snitch gets 50% of victim's bag
- Both snitch → both lose 25%, burned forever

### Anti-Gaming Measures

| Attack Vector | Mitigation |
|--------------|------------|
| Sybil (split wallets) | Minimum 0.1% supply to be eligible; sqrt-weighted selection |
| Collusion (same owner) | On-chain wallet clustering detection |
| Front-running reveals | Commit-reveal with salted hashes |
| MEV extraction | Private mempool via Jito |
| Whale manipulation | Quadratic weighting; max 5% of selections per wallet per month |

---

## 2. TOKEN ECONOMICS

### Supply & Distribution

| Allocation | Amount | % | Vesting |
|-----------|--------|---|---------|
| **Total Supply** | 1,000,000,000 | 100% | — |
| Liquidity Pool | 400,000,000 | 40% | Locked 1 year |
| Community/Airdrop | 250,000,000 | 25% | 50% at launch, 50% over 3 months |
| Yield Treasury | 200,000,000 | 20% | Unlocked (game rewards) |
| Team | 100,000,000 | 10% | 6-month cliff, 12-month linear |
| Marketing/KOL | 50,000,000 | 5% | Released per campaign |

### Tax Structure

| Action | Tax | Destination |
|--------|-----|-------------|
| Buy | 3% | 2% yield pool, 1% marketing |
| Sell | 5% | 3% yield pool, 2% buyback+burn |
| Transfer | 2% | 100% yield pool |

### LP Strategy

1. **Pump.fun Launch** — Graduate to Raydium at ~$300K mcap
2. **Raydium Migration** — Lock LP for 12 months
3. **CEX Listings** — Only if mcap exceeds $5M

---

## 3. WEBSITE/APP

### Pages

```
/                   # Landing — hero, concept explainer, live stats
/play               # Active interrogation view (if user selected)
/drama              # Public feed of all outcomes with wallet tags
/leaderboard        # Top cooperators, top snitches, biggest wins/losses
/profile/[wallet]   # Individual stats, history, reputation score
/rules              # Game mechanics, FAQs
```

### Tech Stack

| Layer | Choice |
|-------|--------|
| Framework | Next.js 14 (App Router) |
| Styling | Tailwind + Framer Motion |
| State | Zustand + React Query |
| Blockchain | @solana/web3.js + Anchor |
| Real-time | Supabase Realtime |
| Hosting | Vercel |

### Design Vibe

**Aesthetic:** Noir interrogation room meets Web3 degen culture
- **Colors:** Deep blacks, blood reds, sickly greens, harsh whites
- **Typography:** Mono fonts (JetBrains Mono, Space Mono)
- **Animations:** Flickering lights, confession cam effects, redacted text reveals

---

## 4. GTM STRATEGY

### Launch Timeline

**Pre-Launch (D-14 to D-1):**
- D-14: Create Twitter, branding, first teaser
- D-10: Announce concept, website teaser
- D-7: Website v1 live
- D-5: Meme contest announced
- D-1: Final countdown

**Launch Week (D0 to D7):**
- D0: LAUNCH on Pump.fun
- D0+2h: First interrogation (use team wallets for guaranteed drama)
- D0+6h: Stream first reveal on Twitter Spaces
- D5: Raydium graduation
- D7: Weekly recap, tease Season 2

### Twitter Strategy

**Handle:** @SnitchOnSolana
**Content:** 40% drama highlights, 30% memes, 15% education, 15% community
**Cadence:** 4-6 tweets/day launch week, 2-3 ongoing

### Growth Loops (CRITICAL)

1. **Auto-tweet outcomes** — Every interrogation result posts to X with wallet tags
2. **Referral mechanic** — Bring victims, get bonus yield
3. **Streamer partnerships** — They play the game live
4. **Daily "WHO SNITCHED" content** — Drama machine
5. **Leaderboards** — Screenshot bait

---

## 5. LAUNCH CHECKLIST

### Pre-Launch
- [ ] Contracts deployed to mainnet
- [ ] LP prepared (50-100 SOL)
- [ ] Website production ready
- [ ] Twitter 500+ followers
- [ ] Discord/TG set up
- [ ] 30+ memes ready
- [ ] KOL deals confirmed

### Launch Day Order
```
T-0:      LAUNCH on Pump.fun
T+5min:   Tweet CA
T+1hr:    First interrogation announcement
T+12hr:   Reveal phase opens
T+24hr:   First outcome = MAXIMUM DRAMA
T+48hr:   Rhythm established
```

---

## 6. COST ESTIMATE

### Lean Launch: ~$26K
- Contracts + audit: $10K
- LP: $3K
- Marketing: $8K
- 3mo ops: $5K

### Standard Launch: ~$58K
- Contracts + audit: $30K
- LP: $5K
- Marketing: $15K
- 3mo ops: $8K

---

## NEXT STEPS

1. Build contracts (Anchor)
2. Build website (Next.js)
3. Set up Twitter bot
4. Create launch assets
5. Deploy and GROW
