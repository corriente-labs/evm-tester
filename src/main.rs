use anyhow::anyhow;
use anyhow::bail;
use glob::glob;
use reader::read_stateful;
use reader::read_stateless;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use string_builder::Builder;

mod core;
mod executor;
mod mover;
mod reader;

use crate::core::{FileType, Output, StateConfig, TestCase, TestGroupConfig, TestCaseSerializable};
use crate::executor::executor::execute;
use crate::mover::mover::to_move_test;

#[allow(dead_code)]
fn write_output(filepath: &str, outputs: &[Output]) -> std::io::Result<()> {
    let file = File::create(filepath)?;
    let text = serde_json::to_string(outputs)?;
    write!(&file, "{}", text)?;
    Ok(())
}

fn write_move_testgroup(test_group_name: &str, filepath: &str, testcases: &[TestCase]) -> anyhow::Result<()> {
    let file = File::create(filepath)?;

    let mut b = Builder::default();
    b.append("#[test_only]\n");
    b.append(format!("module pocvm::{}_tests {{\n", test_group_name));
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

fn write_json_testgroup(_test_group_name: &str, filepath: &str, testcases: &[TestCase]) -> anyhow::Result<()> {
    let file = File::create(filepath)?;
    let testcases = Vec::from(testcases);
    let testcases = testcases.iter().map(|tc| TestCaseSerializable::from(tc));
    let testcases: Vec<TestCaseSerializable>  = testcases.collect();
    let text = serde_json::to_string(&testcases)?;
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
fn read_test_config(path: &str) -> anyhow::Result<TestGroupConfig> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);
    let config: TestGroupConfig = serde_json::from_reader(reader)?;
    Ok(config)
}
fn read_state_config(path: &str) -> anyhow::Result<StateConfig> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);
    let config: StateConfig = serde_json::from_reader(reader)?;
    Ok(config)
}
fn parse_file_type(file_type: &str) -> anyhow::Result<FileType> {
    if file_type.eq_ignore_ascii_case("huff") {
        return Ok(FileType::Huff);
    } else if file_type.eq_ignore_ascii_case("bytecode") {
        return Ok(FileType::Bytecode);
    } else if file_type.eq_ignore_ascii_case("sol") {
        return Ok(FileType::Solidity);
    }
    bail!(format!("unknown test file type {:?}", file_type))
}

fn main() -> anyhow::Result<()> {
    for entry in glob("./resources/**/testcase.json")? {
        if let Ok(path) = entry {
            let mut testcases: Vec<TestCase> = vec![];
            let path = path.display().to_string();
            let config = read_test_config(&path)?;

            let huff_path = path.replace("testcase.json", "*.huff");
            let bc_path = path.replace("testcase.json", "*.bytecode");
            let stateful_path = path.replace("testcase.json", "*/state.json");

            // stateless huff
            for entry in glob(&huff_path)? {
                if let Ok(path) = entry {
                    let test_path = path.display().to_string();

                    let input = read_stateless(&test_path, FileType::Huff)?;
                    let funcname = extract_testname(&test_path)?;
                    let result = execute(input.value, &input.code, &input.calldata, 0, &vec![])?;
                    let testcase = TestCase {
                        funcname,
                        code: result.code,
                        value: result.value,
                        calldata: result.calldata,
                        output: result.output,
                        accounts_input: result.accounts_input,
                        accounts_output: result.accounts_output,
                    };
                    testcases.push(testcase);
                    println!("stateless test case found. {:?}", test_path);
                }
            }
            // stateless bytecode
            for entry in glob(&bc_path)? {
                if let Ok(path) = entry {
                    let test_path = path.display().to_string();

                    let input = read_stateless(&test_path, FileType::Bytecode)?;
                    let funcname = extract_testname(&test_path)?;
                    let result = execute(input.value, &input.code, &input.calldata, 0, &vec![])?;
                    let testcase = TestCase {
                        funcname,
                        code: result.code,
                        value: result.value,
                        calldata: result.calldata,
                        output: result.output,
                        accounts_input: result.accounts_input,
                        accounts_output: result.accounts_output,
                    };
                    testcases.push(testcase);
                    println!("stateless test case found. {:?}", test_path);
                }
            }
            // stateful huff
            for entry in glob(&stateful_path)? {
                if let Ok(path) = entry {
                    let path = path.display().to_string();
                    let state_config = read_state_config(&path)?;
                    let test_path = path.replace("state.json", &state_config.filename);
                    let file_type = parse_file_type(&state_config.filetype)?;

                    let input = read_stateful(&test_path, file_type, &state_config)?;
                    let result = execute(
                        input.value,
                        &input.code,
                        &input.calldata,
                        0,
                        &input.accounts,
                    )?;
                    let testcase = TestCase {
                        funcname: input.id,
                        code: result.code,
                        value: result.value,
                        calldata: result.calldata,
                        output: result.output,
                        accounts_input: result.accounts_input,
                        accounts_output: result.accounts_output,
                    };
                    testcases.push(testcase);
                    println!("stateful test case found. {:?}", test_path);
                }
            }
            let move_path = format!("artifacts/move/{}.move", &config.name);
            write_move_testgroup(&config.name, &move_path, &testcases)?;

            let json_path = format!("artifacts/json/{}.json", &config.name);
            write_json_testgroup(&config.name, &json_path, &testcases)?;
        }
    }
    Ok(())
}
