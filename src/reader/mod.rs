use anyhow::*;
use huff_core::Compiler;
use std::{fs, sync::Arc};

use crate::core::{FileType, Input, StateConfig};

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
    state_config: &StateConfig,
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

    let code = hex::decode(bytecode)?;
    let calldata = hex::decode(&state_config.calldata)?;

    let input = Input {
        id: state_config.id.to_owned(),
        code,
        value: state_config.value,
        calldata,
        accounts: state_config.accounts.clone(),
    };
    Ok(input)
}
