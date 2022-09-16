use glob::glob;
use reader::read_stateful;
use reader::read_stateless;
use std::fs::File;
use std::io::prelude::*;
use string_builder::Builder;

mod core;
mod executor;
mod mover;
mod reader;

use crate::core::{FileType, Output, TestCase};
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
fn extract_testname(path: &str) -> String {
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

            let res = execute(&input.code, &input.calldata, input.value);
            let testcase = TestCase {
                id: testname.to_owned(),
                code: input.code,
                value: input.value,
                calldata: input.calldata,
                output: res.clone(),
            };

            let move_path = src_path
                .replace(".huff", "_test.move")
                .replace("resources/huff", "artifacts/move");
            write_move_test(&testname, &move_path, &[testcase]).unwrap();
            println!("huff test case found. {:?} -> {:?}", src_path, move_path);
        }
    }

    // read stateless bytecode
    for entry in glob("./resources/bytecode/*.bytecode").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let testname = filename.to_owned().replace(".bytecode", "");

            let input = read_stateless(&src_path, FileType::Bytecode).unwrap();

            let res = execute(&input.code, &input.calldata, input.value);
            let testcase = TestCase {
                id: testname.to_owned(),
                code: input.code,
                value: input.value,
                calldata: input.calldata,
                output: res.clone(),
            };

            let move_path = src_path
                .replace(".bytecode", "_test.move")
                .replace("resources/bytecode", "artifacts/move");
            write_move_test(&testname, &move_path, &[testcase]).unwrap();
            println!(
                "bytecode test case found. {:?} -> {:?}",
                src_path, move_path
            );
        }
    }

    // read stateful huff
    for entry in glob("./resources/huff/*/*.huff").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let json_path = src_path.to_owned().replace(filename, "state.json");

            let input = read_stateful(&src_path, FileType::Huff, &json_path).unwrap();

            let res = execute(&input.code, &input.calldata, input.value);
            let testcase = TestCase {
                id: input.id,
                code: input.code,
                value: input.value,
                calldata: input.calldata,
                output: res.clone(),
            };

            let move_path = format!("artifacts/move/{}_test.move", testcase.id);
            println!(
                "stateful huff test case found. {:?} -> {:?}",
                src_path, move_path
            );
            write_move_test(&testcase.id.clone(), &move_path, &[testcase]).unwrap();
        }
    }

    // read stateful bytecode
    for entry in glob("./resources/bytecode/*/*.bytecode").unwrap() {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let json_path = src_path.to_owned().replace(filename, "state.json");

            let input = read_stateful(&src_path, FileType::Bytecode, &json_path).unwrap();

            let res = execute(&input.code, &input.calldata, input.value);
            let testcase = TestCase {
                id: input.id,
                code: input.code,
                value: input.value,
                calldata: input.calldata,
                output: res.clone(),
            };

            let move_path = format!("artifacts/move/{}_test.move", testcase.id);
            println!(
                "stateful bytecode test case found. {:?} -> {:?}",
                src_path, move_path
            );
            write_move_test(&testcase.id.clone(), &move_path, &[testcase]).unwrap();
        }
    }
}
