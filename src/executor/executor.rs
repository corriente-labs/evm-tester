use evm::backend::Backend;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::core::{AccountDeseriarizable, NormalizedAccount};
use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackState, StackSubstateMetadata};
use evm::Config;
use primitive_types::{H160, H256, U256};

#[derive(Debug)]
pub(crate) struct ExecutionResult {
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub output: Vec<u8>,
    pub accounts_input: Vec<NormalizedAccount>,
    pub accounts_output: Vec<NormalizedAccount>,
    pub result: evm::ExitReason,
}

pub(crate) fn execute(
    value: u128,
    code: &[u8],
    calldata: &[u8],
    balance: u128,
    accounts: &[AccountDeseriarizable],
) -> anyhow::Result<ExecutionResult> {
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

    let caller_address = H160::from_str("0xf000000000000000000000000000000000000000")?;
    let dest_address = H160::from_str("0x1000000000000000000000000000000000000000")?;

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
        let normal_acct: NormalizedAccount = acct.into();
        state.insert(
            normal_acct.address,
            MemoryAccount {
                nonce: normal_acct.nonce,
                balance: normal_acct.balance,
                storage: normal_acct.storage.clone(),
                code: normal_acct.code.clone(),
            },
        );
        accounts_input.push(normal_acct);
    }

    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    let (reason, res) = executor.transact_call(
        caller_address,
        dest_address,
        U256::from(value),
        Vec::from(calldata),
        u64::MAX,
        Vec::new(),
    );

    access_accounts(&mut executor.state_mut().metadata_mut(), &accounts_input);

    let mut accounts_output = vec![];
    let state = executor.state();
    let metadata = state.metadata();
    if let Some(accessed) = metadata.accessed() {
        let mut acct_tree: BTreeMap<H160, Vec<(H256, H256)>> = BTreeMap::new();
        for (addr, key) in &accessed.accessed_storage {
            let val = state.storage(*addr, *key);
            if let Some(vec) = acct_tree.get_mut(addr) {
                vec.push((*key, val));
            } else {
                acct_tree.insert(*addr, vec![(*key, val)]);
            }
        }

        // set storages for accounts whose storages are accessed.
        for (addr, storage) in &acct_tree {
            let addr = *addr;
            let mut btree = BTreeMap::new();
            for (key, val) in storage {
                btree.insert(*key, *val);
            }

            let balance = state.basic(addr).balance;
            let nonce = state.basic(addr).nonce;
            let code = state.code(addr);

            let normal_acct = NormalizedAccount {
                address: addr,
                balance,
                nonce,
                code,
                storage: btree,
            };
            accounts_output.push(normal_acct);
        }

        // set account data for accounts who are accessed.
        for addr in &accessed.accessed_addresses {
            if !acct_tree.contains_key(&addr) {
                let addr = *addr;
                let balance = state.basic(addr).balance;
                let nonce = state.basic(addr).nonce;
                let code = state.code(addr);
                let normal_acct = NormalizedAccount {
                    address: addr,
                    balance,
                    nonce,
                    code,
                    storage: BTreeMap::new(),
                };
                accounts_output.push(normal_acct);
            }
        }
    }

    Ok(ExecutionResult {
        code: Vec::from(code),
        value,
        calldata: Vec::from(calldata),
        output: res,
        accounts_input,
        accounts_output,
        result: reason,
    })
}

fn access_accounts(metadata: &mut StackSubstateMetadata, accounts: &[NormalizedAccount]) {
    for acct in accounts {
        metadata.access_address(acct.address);
        let addr_keys = acct.storage.keys().map(|k| (acct.address, *k));
        metadata.access_storages(addr_keys);
    }
}
