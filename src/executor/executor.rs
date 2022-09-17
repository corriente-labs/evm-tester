use evm::backend::Backend;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::core::{Account, NormalizedAccount, TestCase};
use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
use evm::Config;
use primitive_types::{H160, H256, U256};

pub(crate) fn execute(
    id: &str,
    value: u128,
    code: &[u8],
    calldata: &[u8],
    balance: u128,
    accounts: &[Account],
) -> TestCase {
    let config = Config::london();

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
    };

    let caller_address = H160::from_str("0xf000000000000000000000000000000000000000").unwrap();
    let dest_address = H160::from_str("0x1000000000000000000000000000000000000000").unwrap();

    let mut state = BTreeMap::new();
    state.insert(
        dest_address,
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(balance),
            storage: BTreeMap::new(),
            code: Vec::from(code),
        },
    );
    state.insert(
        caller_address,
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    let mut accounts_input = vec![];
    for acct in accounts {
        let mut btree = BTreeMap::new();
        for (key, value) in &acct.storage {
            let key = str_to_H256(&key);
            let value = str_to_H256(&value);
            btree.insert(key, value);
        }
        let address = str_to_H160(&acct.address);

        let normal_acct = NormalizedAccount {
            address,
            balance: U256::from(acct.balance),
            nonce: U256::from(acct.nonce),
            code: hex::decode(&acct.code).unwrap(),
            storage: btree,
        };

        accounts_input.push(normal_acct.clone());

        state.insert(
            normal_acct.address,
            MemoryAccount {
                nonce: normal_acct.nonce,
                balance: normal_acct.balance,
                storage: normal_acct.storage,
                code: normal_acct.code,
            },
        );
    }

    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    let (_reason, res) = executor.transact_call(
        caller_address,
        dest_address,
        U256::from(value),
        Vec::from(calldata),
        u64::MAX,
        Vec::new(),
    );

    let state = executor.state();

    let mut accounts_output = vec![];
    for acct in &accounts_input {
        let mut btree = BTreeMap::new();
        for (key, _) in &acct.storage {
            let key = key.to_owned();
            let val = state.storage(acct.address, key);
            btree.insert(key, val.to_owned());
        }

        let normal_acct = NormalizedAccount {
            address: acct.address,
            balance: state.basic(acct.address).balance,
            nonce: state.basic(acct.address).nonce,
            code: state.code(acct.address),
            storage: btree,
        };
        accounts_output.push(normal_acct);
    }

    TestCase {
        id: String::from(id),
        code: Vec::from(code),
        value,
        calldata: Vec::from(calldata),
        output: res,
        accounts_input: accounts_input,
        accounts_output: accounts_output,
    }
}

#[allow(non_snake_case)]
fn str_to_H256(src: &str) -> H256 {
    let mut word = [0u8; 32];
    let vec: Vec<u8> = hex::decode(&src).unwrap();
    let length = vec.len();
    for i in 0..length {
        word[31 - length + 1 + i] = vec[i];
    }
    H256::from(&word)
}

#[allow(non_snake_case)]
fn str_to_H160(src: &str) -> H160 {
    let mut word = [0u8; 20];
    let vec: Vec<u8> = hex::decode(&src).unwrap();
    let length = vec.len();
    for i in 0..length {
        word[19 - length + 1 + i] = vec[i];
    }
    H160::from(&word)
}
