use std::{
    io::BufReader,
    fs::File
};
use glob::glob;
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
    for entry in glob("./resources/inputs/*.input.json").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();

            let inputs = read(&src_path).unwrap();
            let mut outputs: Vec<Output> = vec![];
            for input in inputs {
                let code = hex::decode(input.code).unwrap();
                let calldata = hex::decode(input.calldata).unwrap();

                let res = execute(code, calldata, input.value);

                outputs.push(Output { id: input.id, data: hex::encode(res) });
            }

            let dest_path = src_path.replace("input", "output");
            write(&dest_path, &outputs).unwrap();

            println!("{:?} test cases found. {:?} -> {:?}", outputs.len(), src_path, dest_path);
        }
    }    
}
