contract;

use std::{
    address::Address,
    auth::{
        AuthError,
        msg_sender,
    },
    block::timestamp,
    constants::ZERO_B256,
    logging::log,
    result::Result,
    revert::require,
};
use oracle_abi::*;

storage {
    prices: StorageMap<ContractId, Price> = StorageMap {},
    owner: Address = Address::from(ZERO_B256),
}

pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

#[storage(read)]
fn validate_owner() {
    let sender = get_msg_sender_address_or_panic();
    require(storage.owner == sender, "Access denied");
}

impl Oracle for Contract {
    #[storage(read, write)]
    fn initialize(owner: Address) {
        require(storage.owner.into() == ZERO_B256, "Cannot reinitialize");
        storage.owner = owner;
    }

    #[storage(read)]
    fn owner() -> Address {
        storage.owner
    }

    #[storage(read, write)]
    fn set_price(asset_id: ContractId, price: u64) {
        validate_owner();
        storage.prices.insert(asset_id, Price {
            price,
            asset_id,
            last_update: timestamp(),
        });
    }

    #[storage(read, write)]
    fn set_prices(prices: Vec<(ContractId, u64)>) {
        validate_owner();
        let mut i = 0;
        while i < prices.len() {
            let (asset_id, price) = prices.get(i).unwrap();
            storage.prices.insert(asset_id, Price {
                price,
                asset_id,
                last_update: timestamp(),
            });
            i += 1;
        }
    }

    #[storage(read)]
    fn get_price(asset_id: ContractId) -> Price {
        storage.prices.get(asset_id).unwrap_or(Price {
            price: 0,
            asset_id,
            last_update: 0,
        })
    }
}
