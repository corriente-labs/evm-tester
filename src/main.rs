use anyhow::anyhow;
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

fn write_move_test(testname: &str, filepath: &str, testcases: &[TestCase]) -> anyhow::Result<()> {
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

    let text = b.string()?;
    write!(&file, "{}", text)?;
    Ok(())
}

#[allow(dead_code)]
fn extract_testname(path: &str) -> anyhow::Result<String> {
    let filename = path.split("/").last().ok_or(anyhow!("invalid file path"))?;
    let filename = filename.to_owned();
    let testname = filename.split(".").collect::<Vec<&str>>()[0];
    Ok(testname.to_owned())
}

fn main() -> anyhow::Result<()> {
    // read stateless huff
    for entry in glob("./resources/huff/*.huff")? {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(anyhow!("failed to get filename"))?;
            let testname = filename.to_owned().replace(".huff", "");

            let input = read_stateless(&src_path, FileType::Huff)?;

            let result = execute(input.value, &input.code, &input.calldata, 0, &vec![])?;
            let testcase = TestCase {
                description: input.id,
                code: result.code,
                value: result.value,
                calldata: result.calldata,
                output: result.output,
                accounts_input: result.accounts_input,
                accounts_output: result.accounts_output,
            };
            let move_path = src_path
                .replace(".huff", "_test.move")
                .replace("resources/huff", "artifacts/move");
            write_move_test(&testname, &move_path, &[testcase])?;
            println!("huff test case found. {:?} -> {:?}", src_path, move_path);
        }
    }

    // read stateless bytecode
    for entry in glob("./resources/bytecode/*.bytecode")? {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(anyhow!("failed to get filename"))?;
            let testname = filename.to_owned().replace(".bytecode", "");

            let input = read_stateless(&src_path, FileType::Bytecode)?;

            let result = execute(input.value, &input.code, &input.calldata, 0, &vec![])?;
            let testcase = TestCase {
                description: input.id,
                code: result.code,
                value: result.value,
                calldata: result.calldata,
                output: result.output,
                accounts_input: result.accounts_input,
                accounts_output: result.accounts_output,
            };

            let move_path = src_path
                .replace(".bytecode", "_test.move")
                .replace("resources/bytecode", "artifacts/move");
            write_move_test(&testname, &move_path, &[testcase])?;
            println!(
                "bytecode test case found. {:?} -> {:?}",
                src_path, move_path
            );
        }
    }

    // read stateful huff
    for entry in glob("./resources/huff/*/*.huff")? {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(anyhow!("failed to get filename"))?;
            let json_path = src_path.to_owned().replace(filename, "state.json");

            let input = read_stateful(&src_path, FileType::Huff, &json_path)?;

            let result = execute(
                input.value,
                &input.code,
                &input.calldata,
                0,
                &input.accounts,
            )?;
            let testcase = TestCase {
                description: input.id,
                code: result.code,
                value: result.value,
                calldata: result.calldata,
                output: result.output,
                accounts_input: result.accounts_input,
                accounts_output: result.accounts_output,
            };

            let move_path = format!("artifacts/move/{}_test.move", testcase.description);
            println!(
                "stateful huff test case found. {:?} -> {:?}",
                src_path, move_path
            );
            write_move_test(&testcase.description.clone(), &move_path, &[testcase])?;
        }
    }

    // read stateful bytecode
    for entry in glob("./resources/bytecode/*/*.bytecode")? {
        if let Ok(path) = entry {
            let src_path = path.display().to_string();
            let filename = path
                .file_name()
                .unwrap()
                .to_str()
                .ok_or(anyhow!("failed to get filename"))?;
            let json_path = src_path.to_owned().replace(filename, "state.json");

            let input = read_stateful(&src_path, FileType::Bytecode, &json_path)?;

            let result = execute(
                input.value,
                &input.code,
                &input.calldata,
                0,
                &input.accounts,
            )?;
            let testcase = TestCase {
                description: input.id,
                code: result.code,
                value: result.value,
                calldata: result.calldata,
                output: result.output,
                accounts_input: result.accounts_input,
                accounts_output: result.accounts_output,
            };

            let move_path = format!("artifacts/move/{}_test.move", testcase.description);
            println!(
                "stateful bytecode test case found. {:?} -> {:?}",
                src_path, move_path
            );
            write_move_test(&testcase.description.clone(), &move_path, &[testcase])?;
        }
    }
    Ok(())
}
