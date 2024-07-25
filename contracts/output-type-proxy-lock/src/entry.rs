use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    high_level::{load_cell_type_hash, load_script, QueryIter},
};

use crate::error::Error;

pub fn check_owner_output_type(owner_input_type_hash: &[u8]) -> bool {
    let is_owner_mode = QueryIter::new(load_cell_type_hash, Source::Output).any(|cell_type_hash| {
        if let Some(cell_type_hash) = cell_type_hash {
            owner_input_type_hash[..] == cell_type_hash[..]
        } else {
            false
        }
    });
    is_owner_mode
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::Encoding);
    }

    let owner_script_hash = &args[0..32];
    if check_owner_output_type(owner_script_hash) {
        return Ok(());
    }

    return Err(Error::InvalidUnlock);
}
