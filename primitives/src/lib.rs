use codec::{Decode, Encode};
use rstd::vec::Vec;
use sp_core::{RuntimeDebug, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use xcm::v0::Junction;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
pub struct XrecoveryCreateRecoveryCall {
    call_index: [u8; 2],
    friends: OpaqueRequest,
    threshold: u16,
    delay_period: u32,
}

pub type OpaqueRequest = Vec<u8>;

impl XrecoveryCreateRecoveryCall {
    pub fn new(pallet_index: u8, call_index: u8, friends: Vec<u8>, threshold: u16, delay_period: u32) 
    -> Self {
        XrecoveryCreateRecoveryCall {
            call_index: [pallet_index, call_index],
            friends: friends,
            threshold: threshold,
            delay_period: delay_period,
        }
    }
}

