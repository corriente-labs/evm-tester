use std::{
    sync::Arc, fs,
};
use huff_core::Compiler;
use anyhow::*;

use crate::core::{Input, FileType};

pub(crate) fn read_stateless(filepath: &str, filetype: FileType) -> anyhow::Result<Input> {
    let bytecode = match filetype {
        FileType::Huff => {
            let compiler = Compiler::new(Arc::new(vec![filepath.to_owned()]), None, None, None, false, false);
            let res = compiler.execute().unwrap();
            res[0].runtime.to_owned()
        },
        FileType::Solidity => {
            bail!("Solidity not supported.")
        },
        FileType::Bytecode => {
            let data = fs::read_to_string(filepath)?;
            data
        },
    };

    let input = Input {
        id: filepath.to_owned(),
        code: bytecode,
        value: 0,
        calldata: "".to_owned(),
    };
    Ok(input)
}