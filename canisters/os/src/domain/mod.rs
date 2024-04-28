pub mod request;

use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use crate::constants::METADATA_SIZE;

const WALLET_OWNER_MAX_SIZE: u32 = 100;
const WALLET_ACTION_MAX_SIZE: u32 = 100;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Metadata {
    pub network: BitcoinNetwork,
    pub steward_canister: Principal,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            steward_canister: Principal::anonymous(),
            network: BitcoinNetwork::Regtest,
        }
    }
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: METADATA_SIZE as u32,
        is_fixed_size: false,
    };
}

/// The `State` will store the canister info when a user create a wallet.
/// A wallet is also a canister, call `SmartWallet`
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct WalletOwner {
    pub canister_id: Principal,
    pub owner: Principal,
    pub created_at: u64,
}

/// For a type to be used in Stable storage like `StableBtreeMap`, it need to implement the `Storable` trait,
/// which specifies how the type can be serialized/deserialized.
impl Storable for WalletOwner {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WALLET_OWNER_MAX_SIZE,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize)]
pub struct WalletAction {
    pub operator: Principal,
    pub action: Action,
    pub op_time: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Action {
    Create,
    Delete,
}

impl Storable for WalletAction {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WALLET_ACTION_MAX_SIZE,
        is_fixed_size: false,
    };
}

pub struct WalletCanisterDeployArgs {
    // sub_account: Option<Subaccount>,
}
