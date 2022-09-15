use std::{
    io::BufReader,
    fs::File
};
use glob::glob;
use reader::read_stateless;
use string_builder::Builder;
use std::io::prelude::*;

mod core;
mod reader;
mod executor;
mod mover;

use crate::core::{Input, Output, TestCase, FileType};
use crate::executor::executor::execute;
use crate::mover::mover::to_move_test;

#[allow(dead_code)]
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
    b.append("    use std::signer;\n");
    // b.append("    use std::unit_test;\n");
    // b.append("    use std::vector;\n");
    b.append("    use aptos_framework::coin;\n");
    b.append("    use aptos_framework::aptos_coin::{Self, AptosCoin};\n");
    b.append("    use aptos_framework::aptos_account;\n");
    b.append("    use pocvm::vm;\n\n");

    for tc in testcases {
        let s = to_move_test(tc);
        b.append(s);
    }
    b.append("}\n");

    let text = b.string().unwrap();
    write!(&file, "{}", text)?;
    Ok(())
}

#[allow(dead_code)]
fn read(filepath: &str) -> std::io::Result<Vec<Input>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let inputs: Vec<Input> = serde_json::from_reader(reader)?;
    Ok(inputs)
}

#[allow(dead_code)]
fn extract_testname(path: &str)-> String {
    let filename = path.split("/").last().unwrap();
    let filename = filename.to_owned();
    let testname = filename.split(".").collect::<Vec<&str>>()[0];
    testname.to_owned()
}

fn main() {

    // read stateless huff
    for entry in glob("./resources/huff/*.huff").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let testname = filename.to_owned().replace(".huff", "");

            let input = read_stateless(&src_path, FileType::Huff).unwrap();
            
            let code = hex::decode(input.code).unwrap();
            let calldata = hex::decode(input.calldata).unwrap();

            let res = execute(&code, &calldata, input.value);
            let testcase = TestCase {
                id: testname.to_owned(),
                code,
                value: input.value,
                calldata,
                output: res.clone()
            };

            let move_path = src_path.replace(".huff", "_test.move")
                .replace("resources/huff", "artifacts/move");
            write_move_test(&testname, &move_path, &[testcase]).unwrap();
            println!("huff test case found. {:?} -> {:?}", src_path, move_path);
        }
    }

    // read stateful huff
    for entry in glob("./resources/huff/*/*.huff").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            println!("{:?}", src_path);
        }
    }
}
