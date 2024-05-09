use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{bitcoin::BitcoinNetwork, main::CanisterId};
use serde::Deserialize;

use super::StakingPoolInfo;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgument {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub wallet_cycles: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitWalletArgument {
    pub name: String,
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
    pub owner: Option<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateStakingPoolRequest {
    pub name: String,
    pub description: String,
    pub annual_interest_rate: u64,
    pub duration_in_millisecond: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct InitStakingPoolArgument {
    pub name: String,
    pub description: String,
    pub network: BitcoinNetwork,
    pub annual_interest_rate: u64,
    pub duration_in_millisecond: u64,
    pub os_canister: CanisterId,
}

impl From<StakingPoolInfo> for InitStakingPoolArgument {
    fn from(info: StakingPoolInfo) -> Self {
        Self {
            name: info.name,
            description: info.description,
            network: info.network,
            annual_interest_rate: info.annual_interest_rate,
            duration_in_millisecond: info.duration_in_millisecond,
            os_canister: info.os_canister,
        }
    }
}
