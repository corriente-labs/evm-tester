use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub(crate) struct Input {
    pub id: String,
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StateInput {
    pub id: String,
    pub value: u128,
    pub calldata: String,
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Account {
    pub address: String,
    pub balance: u128,
    pub nonce: u128,
    pub code: String,
    pub storage: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Output {
    pub id: String,
    pub data: String,
}

#[derive(Debug)]
pub(crate) struct TestCase {
    pub id: String,
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub output: Vec<u8>,
    pub accounts_input: Vec<NormalizedAccount>,
    pub accounts_output: Vec<NormalizedAccount>,
}

#[derive(Debug, Clone)]
pub(crate) struct NormalizedAccount {
    pub address: H160,
    pub balance: U256,
    pub nonce: U256,
    pub code: Vec<u8>,
    pub storage: BTreeMap<H256, H256>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum FileType {
    Huff,
    Solidity,
    Bytecode,
}
