use std::collections::BTreeMap;
use std::str::FromStr;
use std::{
    io::BufReader,
    fs::File
};
use primitive_types::{U256, H160};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
use evm::Config;

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    id: String,
    code: String,
    value: u128,
    calldata: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Account {
//     address: String,
//     nonce: String,
//     balance: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    id: String,
    data: String,
}

fn write(filepath: &str, outputs: &[Output]) -> std::io::Result<()> {
    let file = File::create(filepath)?;    
    let text = serde_json::to_string(outputs)?;
    write!(&file, "{}", text)?;

    Ok(())
}

fn read(filepath: &str) -> std::io::Result<Vec<Input>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let inputs: Vec<Input> = serde_json::from_reader(reader)?;
    Ok(inputs)
}

fn main() {
    let config = Config::istanbul();

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

    let inputs = read("./resources/inputs/huff.input.json").unwrap();

    let mut outputs: Vec<Output> = vec![];
    for input in inputs {
        println!("{:?}", input);

        let calldata = hex::decode(input.calldata).unwrap();
        let code = hex::decode(input.code).unwrap();

        let mut state = BTreeMap::new();
        state.insert(
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            MemoryAccount {
                nonce: U256::one(),
                balance: U256::from(input.value),
                storage: BTreeMap::new(),
                code,
            }
        );
        state.insert(
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            MemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10000000),
                storage: BTreeMap::new(),
                code: Vec::new(),
            },
        );

        let backend = MemoryBackend::new(&vicinity, state);
        let metadata = StackSubstateMetadata::new(u64::MAX, &config);
        let state = MemoryStackState::new(metadata, &backend);
        let precompiles = BTreeMap::new();
        let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

        let (_reason, res) = executor.transact_call(
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            U256::zero(),
            calldata,
            u64::MAX,
            Vec::new(),
        );

        outputs.push(Output { id: input.id, data: hex::encode(res) });
    }

    write("./resources/outputs/huff.output.json", &outputs).unwrap();
}
