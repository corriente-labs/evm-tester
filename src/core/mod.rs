use serde::{Deserialize, Serialize};

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
    pub storage: String,
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
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum FileType {
    Huff,
    Solidity,
    Bytecode,
}
