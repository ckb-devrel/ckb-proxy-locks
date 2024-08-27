use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    // debug,
    high_level::{load_cell_lock_hash, load_input_since, load_script, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();

    if args.len() < 40 {
        return Err(Error::InvalidArguments);
    }

    let locked_until = &args[0..8];
    if !has_lock_time_expired(locked_until) {
        return Err(Error::TimeLockNotExpired);
    }

    let required_lock_script_hash = &args[8..40];

    if !required_lock_script_exists(required_lock_script_hash) {
        return Err(Error::RequireLockScriptNotFound);
    }

    Ok(())
}

pub fn has_lock_time_expired(locked_until: &[u8]) -> bool {
    for since in QueryIter::new(load_input_since, Source::Input) {
        // compare since with locked_until
        let locked_until_timestamp = u64::from_le_bytes(locked_until.try_into().unwrap());
        if since > locked_until_timestamp {
            return true;
        }
    }
    false
}

pub fn required_lock_script_exists(required_lock_script_hash: &[u8]) -> bool {
    QueryIter::new(load_cell_lock_hash, Source::Input)
        .any(|cell_lock_hash| required_lock_script_hash[..] == cell_lock_hash[..])
}
