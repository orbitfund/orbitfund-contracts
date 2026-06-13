# 🪐 OrbitFund Contracts

Soroban smart contracts powering OrbitFund — decentralized milestoned crowdfunding on Stellar. Instead of releasing 100% of funds to a creator on successful funding, capital is locked in an on-chain escrow and released in incremental tiers only after backers vote to approve verifiable creator milestones. This eliminates rug-pulls at the protocol level.

---

## How It Works

```
  Backer A ──┐
  Backer B ──┼──► pledge() ──► [Campaign Escrow Contract]
  Backer C ──┘                        │
                                       │ creator submits proof
                                       ▼
                              submit_milestone_proof()
                                       │
                                       │ voting window opens
                                       ▼
                              vote_on_milestone()  ◄── Backers vote
                              (weight = pledge amount)
                                       │
                          ┌────────────┴────────────┐
                          │ votes_for > votes_against│
                          ▼                          ▼
                   resolve_milestone()         resolve_milestone()
                   → transfer allocation%      → CampaignState::Failed
                     of escrow to creator        → clawbacks enabled
```

A `factory` contract handles deploying individual campaign instances so each campaign is fully isolated — one contract address per campaign, no shared state.

---

## Architecture

### Contract Crates

| Crate | File | Purpose |
|---|---|---|
| `campaign` | `contracts/campaign/src/lib.rs` | Core escrow, pledging, milestone voting, fund release |
| `factory` | `contracts/factory/src/lib.rs` | Deploys new `campaign` instances with a deterministic address |

### State Machine

Each campaign moves through a strict set of states:

```
Funding ──► Active ──► Succeeded
                └────► Failed
```

| State | Meaning |
|---|---|
| `Funding` | Accepts pledges; deadline not yet passed |
| `Active` | Funding goal met; milestones being voted on |
| `Succeeded` | All milestones approved; all funds released to creator |
| `Failed` | A milestone was rejected; backers may claim refunds |

### Storage Layout (`DataKey`)

All campaign state lives in **instance storage** on the campaign contract:

| Key | Type | Description |
|---|---|---|
| `Creator` | `Address` | Campaign creator; authorized to submit milestone proofs |
| `Token` | `Address` | Stellar Asset Contract address used for pledges |
| `TargetAmount` | `u128` | Funding goal in token base units |
| `Deadline` | `u64` | Ledger timestamp after which pledges are rejected |
| `CurrentState` | `CampaignState` | Current phase of the campaign |
| `Milestones` | `Vec<Milestone>` | Ordered list of funding tiers with voting data |
| `Backers` | `Map<Address, u128>` | Per-backer pledge totals (used for voting weight) |

### Milestone Struct

```rust
pub struct Milestone {
    pub description: Symbol,          // Short label, e.g. "MVP Launch"
    pub allocation_percentage: u32,   // % of total escrow unlocked on approval (sum must = 100)
    pub approved: bool,               // Set to true after a successful vote
    pub votes_for: u128,              // Cumulative voting weight in favour
    pub votes_against: u128,          // Cumulative voting weight against
    pub voting_deadline: u64,         // Ledger timestamp; voting closes after this
}
```

---

## Repo Structure

```
orbitfund-contracts/
├── .github/
│   ├── workflows/ci.yml                    # PR gate: fmt → clippy → test → wasm build
│   └── ISSUE_TEMPLATE/smart-contract-task.md
├── contracts/
│   ├── campaign/
│   │   ├── Cargo.toml                      # soroban-sdk = "26.0.1"
│   │   └── src/lib.rs                      # ← primary contribution target
│   └── factory/
│       ├── Cargo.toml
│       └── src/lib.rs
├── Cargo.toml                              # Workspace (resolver = "2")
├── rust-toolchain.toml                     # stable + wasm32-unknown-unknown
├── rustfmt.toml                            # edition = "2021"
├── CONTRIBUTING.md
└── README.md
```

---

## Contract API Reference

### `campaign` — `OrbitFundCampaign`

#### `initialize`
Sets up a new campaign. Called once by the factory immediately after deployment.

```rust
pub fn initialize(
    env: Env,
    creator: Address,      // Campaign owner
    token: Address,        // SAC token for pledges (e.g. USDC)
    target_amount: u128,   // Funding goal in base units
    deadline: u64,         // Ledger timestamp cutoff for pledges
    milestones: Vec<Milestone>,
)
```

#### `pledge`
Transfers tokens from a backer into the contract's escrow.

```rust
pub fn pledge(env: Env, backer: Address, amount: u128)
// Requires: CampaignState == Funding, timestamp <= Deadline
// Effect: token transfer backer → contract; updates Backers map
```

#### `submit_milestone_proof`
Creator signals a milestone is complete and opens a voting window.

```rust
pub fn submit_milestone_proof(env: Env, milestone_index: u32, proof_url: Symbol)
// Requires: creator auth; milestone not yet approved
// Effect: sets milestone.voting_deadline, transitions to Active
```

#### `vote_on_milestone`
Backers cast votes; weight is proportional to their total pledge.

```rust
pub fn vote_on_milestone(env: Env, backer: Address, milestone_index: u32, approve: bool)
// Requires: backer auth; within voting_deadline
// Effect: increments votes_for or votes_against
```

#### `resolve_milestone`
Called by anyone after voting closes. Releases funds or triggers failed state.

```rust
pub fn resolve_milestone(env: Env, milestone_index: u32)
// Requires: timestamp >= milestone.voting_deadline
// Effect (pass): transfer allocation_percentage of escrow to creator
// Effect (fail): set CampaignState::Failed; enable backer clawbacks
```

### `factory` — `OrbitFundFactory`

#### `deploy`
Deploys a new `campaign` contract instance with a deterministic salt.

```rust
pub fn deploy(
    env: Env,
    campaign_wasm_hash: BytesN<32>,  // Uploaded campaign wasm hash
    salt: BytesN<32>,                // Unique salt per campaign
    deployer: Address,               // Becomes campaign creator
) -> Address                         // Returns deployed contract address
```

---

## Local Setup

**Prerequisites:** Rust stable toolchain + `stellar-cli`.

```bash
# Install wasm target (once)
rustup target add wasm32-unknown-unknown

# Build all contracts to wasm
cargo build --target wasm32-unknown-unknown --release

# Run unit tests (uses soroban testutils, no network needed)
cargo test --all

# Lint — must be zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --all -- --check
```

### Install `stellar-cli` (for testnet deployment)

```bash
cargo install --locked stellar-cli --features opt
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

---

## Writing a Test

Tests use `soroban-sdk`'s built-in mock environment — no node or testnet required.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, vec, Env, Symbol};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register(OrbitFundCampaign, ());
        let client = OrbitFundCampaignClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let token = Address::generate(&env);
        let milestones = vec![
            &env,
            Milestone {
                description: Symbol::new(&env, "mvp"),
                allocation_percentage: 100,
                approved: false,
                votes_for: 0,
                votes_against: 0,
                voting_deadline: 0,
            },
        ];

        client.initialize(&creator, &token, &10_000_u128, &9_999_999_u64, &milestones);

        // Assert state was written correctly
        env.as_contract(&contract_id, || {
            let stored: Address = env.storage().instance().get(&DataKey::Creator).unwrap();
            assert_eq!(stored, creator);
        });
    }
}
```

All new contributions must include tests following this pattern. Run them with:

```bash
cargo test --all -- --nocapture
```

---

## Contributors

Issues in this repo map directly to GrantFox bounty campaigns. To pick up work:

1. Browse open, unassigned GitHub issues tagged `smart-contract`.
2. Confirm your local environment compiles and `cargo test --all` passes.
3. Implement the function described in the issue. Follow the acceptance criteria exactly.
4. Open a PR with the title format: `feat/<issue-number>: <short description>`.
5. PRs must pass all CI gates (fmt → clippy → test → wasm build) to qualify for rewards.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for branch naming, PR rules, and coding standards.

---

## Tech Stack

| Layer | Choice | Version |
|---|---|---|
| Language | Rust | stable |
| Smart Contract SDK | soroban-sdk | `26.0.1` |
| Compile Target | wasm32-unknown-unknown | — |
| Test Framework | soroban-sdk testutils | `26.0.1` |
| Linter | clippy | stable |
| Formatter | rustfmt | edition 2021 |
| Network | Stellar Testnet / Mainnet | Protocol 25+ |
