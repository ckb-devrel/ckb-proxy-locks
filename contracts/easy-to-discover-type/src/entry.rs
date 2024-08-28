use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    high_level::{load_cell_data_hash, load_script, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::InsufficientArgsLength);
    }

    let data_hash = &args[0..32];

    if QueryIter::new(load_cell_data_hash, Source::GroupOutput).any(|hash| data_hash != hash) {
        return Err(Error::DataHashNotMatch);
    }

    Ok(())
}
