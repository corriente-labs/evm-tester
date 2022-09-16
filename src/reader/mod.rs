use anyhow::*;
use huff_core::Compiler;
use std::{fs, sync::Arc};

use crate::core::{FileType, Input};

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

    let input = Input {
        id: filepath.to_owned(),
        code: bytecode,
        value: 0,
        calldata: "".to_owned(),
    };
    Ok(input)
}
