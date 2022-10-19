use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub(crate) struct Input {
    pub id: String,
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub accounts: Vec<AccountDeseriarizable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TestGroupConfig {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct StateConfig {
    pub id: String,
    pub filename: String,
    pub filetype: String,
    pub value: u128,
    pub calldata: String,
    pub accounts: Vec<AccountDeseriarizable>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AccountDeseriarizable {
    pub address: String,
    pub balance: u128,
    pub nonce: u128,
    pub code: String,
    pub storage: HashMap<String, String>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct AccountSeriarizable {
    pub address: H160,
    pub balance: U256,
    pub nonce: U256,
    pub code: String,
    pub storage: BTreeMap<H256, H256>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Output {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TestCase {
    pub funcname: String,
    pub code: Vec<u8>,
    pub value: u128,
    pub calldata: Vec<u8>,
    pub output: Vec<u8>,
    pub accounts_input: Vec<NormalizedAccount>,
    pub accounts_output: Vec<NormalizedAccount>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TestCaseSerializable {
    pub funcname: String,
    pub code: String,
    pub value: U256,
    pub calldata: String,
    pub output: String,
    pub accounts_input: Vec<AccountSeriarizable>,
    pub accounts_output: Vec<AccountSeriarizable>,
}

impl From<&TestCase> for TestCaseSerializable {
    fn from(tc: &TestCase) -> Self {
        return TestCaseSerializable {
            funcname: tc.funcname.to_owned(),
            code: hex::encode(tc.code.to_owned()),
            value: U256::from(tc.value),
            calldata: hex::encode(tc.calldata.to_owned()),
            output: hex::encode(tc.output.to_owned()),
            accounts_input: tc.accounts_input.iter().map(|acct| AccountSeriarizable {
                address: acct.address,
                balance: acct.balance,
                nonce: acct.nonce,
                code: hex::encode(&acct.code),
                storage: acct.storage.to_owned(),
            }).collect(),
            accounts_output: tc.accounts_output.iter().map(|acct| AccountSeriarizable {
                address: acct.address,
                balance: acct.balance,
                nonce: acct.nonce,
                code: hex::encode(&acct.code),
                storage: acct.storage.to_owned(),
            }).collect(), 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NormalizedAccount {
    pub address: H160,
    pub balance: U256,
    pub nonce: U256,
    pub code: Vec<u8>,
    pub storage: BTreeMap<H256, H256>,
}

impl From<AccountDeseriarizable> for NormalizedAccount {
    fn from(acct: AccountDeseriarizable) -> Self {
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
impl From<&AccountDeseriarizable> for NormalizedAccount {
    fn from(acct: &AccountDeseriarizable) -> Self {
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

// impl From<NormalizedAccount> for AccountSeriarizable {
//     fn from(acct: NormalizedAccount) -> Self {
//         let mut btree = BTreeMap::new();
//         for (key, value) in &acct.storage {
//             let key = str_to_H256(&key);
//             let value = str_to_H256(&value);
//             btree.insert(key, value);
//         }
//     }
// }

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
