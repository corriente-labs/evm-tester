use anyhow::*;
use huff_core::Compiler;
use std::{fs, sync::Arc};
use std::{fs::File, io::BufReader};

use crate::core::{FileType, Input, StateInput};

fn strip_non_hex_chars(data: &str) -> String {
    let original = data.to_owned();
    original.replace("\n", "")
}

pub(crate) fn read_stateless(filepath: &str, filetype: FileType) -> anyhow::Result<Input> {
    let bytecode = match filetype {
        FileType::Huff => {
            let compiler = Compiler::new(
                Arc::new(vec![filepath.to_owned()]),
                None,
                None,
                None,
                false,
                false,
            );
            let res = compiler.execute().unwrap();
            res[0].runtime.to_owned()
        }
        FileType::Solidity => {
            bail!("Solidity not supported.")
        }
        FileType::Bytecode => {
            let data = fs::read_to_string(filepath)?;
            strip_non_hex_chars(&data)
        }
    };

    let code = hex::decode(bytecode)?;
    let calldata = hex::decode("")?;

    let input = Input {
        id: filepath.to_owned(),
        code,
        value: 0,
        calldata,
        accounts: vec![],
    };
    Ok(input)
}

pub(crate) fn read_stateful(
    filepath: &str,
    filetype: FileType,
    jsonpath: &str,
) -> anyhow::Result<Input> {
    let bytecode = match filetype {
        FileType::Huff => {
            let compiler = Compiler::new(
                Arc::new(vec![filepath.to_owned()]),
                None,
                None,
                None,
                false,
                false,
            );
            let res = compiler.execute().unwrap();
            res[0].runtime.to_owned()
        }
        FileType::Solidity => {
            bail!("Solidity not supported.")
        }
        FileType::Bytecode => {
            let data = fs::read_to_string(filepath)?;
            strip_non_hex_chars(&data)
        }
    };

    let json_file = File::open(jsonpath)?;
    let reader = BufReader::new(json_file);
    let state_input: StateInput = serde_json::from_reader(reader)?;

    let code = hex::decode(bytecode)?;
    let calldata = hex::decode(state_input.calldata)?;

    let input = Input {
        id: state_input.id,
        code,
        value: state_input.value,
        calldata,
        accounts: state_input.accounts,
    };
    Ok(input)
}
