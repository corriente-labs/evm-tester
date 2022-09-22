use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub(crate) struct Input {
    pub id: String,
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub accounts: Vec<AccountSeriarizable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TestGroupConfig {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StateConfig {
    pub id: String,
    pub filename: String,
    pub filetype: String,
    pub value: u128,
    pub calldata: String,
    pub accounts: Vec<AccountSeriarizable>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AccountSeriarizable {
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
    pub funcname: String,
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

impl From<AccountSeriarizable> for NormalizedAccount {
    fn from(acct: AccountSeriarizable) -> Self {
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

        normal_acct
    }
}
impl From<&AccountSeriarizable> for NormalizedAccount {
    fn from(acct: &AccountSeriarizable) -> Self {
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

        normal_acct
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum FileType {
    Huff,
    Solidity,
    Bytecode,
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
