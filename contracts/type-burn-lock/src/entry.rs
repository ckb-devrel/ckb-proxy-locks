use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    high_level::{load_cell_type_hash, load_script, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::Encoding);
    }

    let expected_type_hash = &args[..32];
    let find_input = QueryIter::new(load_cell_type_hash, Source::Input).any(|type_hash| {
        if let Some(hash) = type_hash {
            hash == expected_type_hash
        } else {
            false
        }
    });
    let find_output = QueryIter::new(load_cell_type_hash, Source::Output).any(|type_hash| {
        if let Some(hash) = type_hash {
            hash == expected_type_hash
        } else {
            false
        }
    });

    if !find_input || find_output {
        return Err(Error::TypeScriptNotBurnt);
    }

    Ok(())
}
