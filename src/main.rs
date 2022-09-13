use std::{
    io::BufReader,
    fs::File
};
use glob::glob;
use string_builder::Builder;
use std::io::prelude::*;

mod core;
mod executor;
mod mover;

use crate::core::{Input, Output, TestCase};
use crate::executor::executor::execute;
use crate::mover::mover::to_move_test;

fn write_output(filepath: &str, outputs: &[Output]) -> std::io::Result<()> {
    let file = File::create(filepath)?;
    let text = serde_json::to_string(outputs)?;
    write!(&file, "{}", text)?;
    Ok(())
}

fn write_move_test(testname: &str, filepath: &str, testcases: &[TestCase]) -> std::io::Result<()> {
    let file = File::create(filepath)?;

    let mut b = Builder::default();
    b.append("#[test_only]\n");
    println!("{}", testname);
    b.append(format!("module pocvm::{}_tests {{\n", testname));
    b.append("  use std::signer;\n");
    b.append("  use std::unit_test;\n");
    b.append("  use std::vector;\n");
    b.append("  use aptos_framework::coin;\n");
    b.append("  use aptos_framework::aptos_coin::{Self, AptosCoin};\n");
    b.append("  use aptos_framework::aptos_account;\n");
    b.append("  use pocvm::vm;\n\n");

    for tc in testcases {
        let s = to_move_test(tc);
        b.append(s);
    }

    let text = b.string().unwrap();
    write!(&file, "{}", text)?;
    Ok(())
}

fn read(filepath: &str) -> std::io::Result<Vec<Input>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let inputs: Vec<Input> = serde_json::from_reader(reader)?;
    Ok(inputs)
}

fn extract_testname(path: &str)-> String {
    let filename = path.split("/").last().unwrap();
    let filename = filename.to_owned();
    let testname = filename.split(".").collect::<Vec<&str>>()[0];
    testname.to_owned()
}

fn main() {
    for entry in glob("./resources/inputs/*.input.json").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();

            let inputs = read(&src_path).unwrap();
            let mut outputs: Vec<Output> = vec![];
            let mut testcases: Vec<TestCase> = vec![];
            for input in inputs {
                let code = hex::decode(input.code).unwrap();
                let calldata = hex::decode(input.calldata).unwrap();

                let res = execute(&code, &calldata, input.value);

                outputs.push(Output { id: input.id.clone(), data: hex::encode(&res) });
                testcases.push(TestCase { id: input.id.clone(), code: code, value: input.value, calldata: calldata, output: res.clone() });
            }

            let testname = extract_testname(&src_path);
            let output_path = src_path.replace("input", "output");
            write_output(&output_path, &outputs).unwrap();

            let move_path = src_path.replace(".input", "_test")
                .replace("json", "move")
                .replace("resources/inputs", "move");
            write_move_test(&testname, &move_path, &testcases).unwrap();

            println!("{:?} test cases found. {:?} -> {:?}", outputs.len(), src_path, output_path);
        }
    }    
}
