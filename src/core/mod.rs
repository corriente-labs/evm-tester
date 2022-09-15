use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Input {
    pub id: String,
    pub code: String,
    pub value: u128,
    pub calldata: String,
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
