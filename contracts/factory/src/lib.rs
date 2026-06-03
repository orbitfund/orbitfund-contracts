#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct OrbitFundFactory;

#[contractimpl]
impl OrbitFundFactory {
    /// Deploy a new OrbitFundCampaign contract instance.
    /// Returns the deployed contract's address.
    pub fn deploy(
        env: Env,
        campaign_wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        deployer: Address,
    ) -> Address {
        deployer.require_auth();
        let _ = (&campaign_wasm_hash, &salt);
        // TODO: Use env.deployer().with_address(deployer, salt).deploy(campaign_wasm_hash)
        // TODO: Call campaign.initialize(...) with creator args after deployment.
        let _ = &env;
        deployer
    }
}
