#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CampaignState {
    Funding,
    Active,
    Succeeded,
    Failed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub description: Symbol,
    pub allocation_percentage: u32,
    pub approved: bool,
    pub votes_for: u128,
    pub votes_against: u128,
    pub voting_deadline: u64,
}

#[contracttype]
pub enum DataKey {
    Creator,
    Token,
    TargetAmount,
    Deadline,
    CurrentState,
    Milestones,
    Backers,
}

#[contract]
pub struct OrbitFundCampaign;

#[contractimpl]
impl OrbitFundCampaign {
    /// Initialize a new milestoned crowdfunding campaign.
    pub fn initialize(
        env: Env,
        creator: Address,
        token: Address,
        target_amount: u128,
        deadline: u64,
        milestones: Vec<Milestone>,
    ) {
        env.storage().instance().set(&DataKey::Creator, &creator);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::TargetAmount, &target_amount);
        env.storage().instance().set(&DataKey::Deadline, &deadline);
        env.storage().instance().set(&DataKey::CurrentState, &CampaignState::Funding);
        env.storage().instance().set(&DataKey::Milestones, &milestones);
    }

    /// Backers deposit funds into the campaign.
    pub fn pledge(env: Env, backer: Address, amount: u128) {
        backer.require_auth();
        let _ = amount;
        // TODO: Transfer `amount` of Token from backer to contract via token client.
        // TODO: Load Backers map, add/increment backer's entry, and save it back.
        // TODO: Reject pledge if CurrentState != Funding or env.ledger().timestamp() > Deadline.
        let _ = &env;
    }

    /// Creator submits proof for a completed milestone to open the voting window.
    pub fn submit_milestone_proof(env: Env, milestone_index: u32, proof_url: Symbol) {
        let creator: Address = env.storage().instance().get(&DataKey::Creator).unwrap();
        creator.require_auth();
        let _ = (milestone_index, proof_url);
        // TODO: Load Milestones vec, verify milestone_index is in bounds and not yet approved.
        // TODO: Set milestone.voting_deadline = env.ledger().timestamp() + VOTING_PERIOD.
        // TODO: Transition CurrentState to Active.
    }

    /// Backers vote on a milestone; weight is proportional to their pledged amount.
    pub fn vote_on_milestone(env: Env, backer: Address, milestone_index: u32, approve: bool) {
        backer.require_auth();
        let _ = (milestone_index, approve);
        // TODO: Load Backers map; derive voting weight from backer's deposit.
        // TODO: Increment votes_for or votes_against on the specified milestone.
        // TODO: Reject if voting_deadline has passed.
        let _ = &env;
    }

    /// Resolve a milestone after its voting deadline: release funds or trigger refunds.
    pub fn resolve_milestone(env: Env, milestone_index: u32) {
        let _ = milestone_index;
        // TODO: Verify env.ledger().timestamp() >= milestone.voting_deadline.
        // TODO: If votes_for > votes_against: transfer allocation_percentage of funds to creator.
        // TODO: If milestone fails repeatedly: set CurrentState to Failed to allow clawbacks.
        let _ = &env;
    }
}

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
        let milestones: soroban_sdk::Vec<Milestone> = vec![
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

        env.as_contract(&contract_id, || {
            let stored: Address = env.storage().instance().get(&DataKey::Creator).unwrap();
            assert_eq!(stored, creator);
            let state: CampaignState =
                env.storage().instance().get(&DataKey::CurrentState).unwrap();
            assert_eq!(state, CampaignState::Funding);
        });
    }
}
