use std::{
    io::BufReader,
    fs::File
};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

mod executor;

use crate::executor::executor::execute;

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    id: String,
    code: String,
    value: u128,
    calldata: String,
}

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
    

    let inputs = read("./resources/inputs/huff.input.json").unwrap();

    let mut outputs: Vec<Output> = vec![];
    for input in inputs {
        println!("{:?}", input);

        let calldata = hex::decode(input.calldata).unwrap();
        let code = hex::decode(input.code).unwrap();

        let res = execute(code, calldata, input.value);

        outputs.push(Output { id: input.id, data: hex::encode(res) });
    }

    write("./resources/outputs/huff.output.json", &outputs).unwrap();
}
