use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    high_level::{load_cell_lock_hash, load_script, QueryIter},
};

use crate::error::Error;

pub fn check_owner_lock(owner_lock_hash: &[u8]) -> bool {
    let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
        .any(|cell_lock_hash| owner_lock_hash[..] == cell_lock_hash[..]);
    is_owner_mode
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::Encoding);
    }

    let owner_script_hash = &args[0..32];
    if check_owner_lock(owner_script_hash) {
        return Ok(());
    }

    return Err(Error::InvalidUnlock);
}
