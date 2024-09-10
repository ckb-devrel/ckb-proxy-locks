use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::Unpack,
    // debug,
    high_level::{load_cell_lock_hash, load_input_since, load_script, QueryIter},
    since::Since,
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();

    if args.len() < 40 {
        return Err(Error::InvalidArguments);
    }

    let required_lock_script_hash = &args[..32];

    if !required_lock_script_exists(required_lock_script_hash) {
        return Err(Error::RequiredLockScriptNotFound);
    }

    let locked_until = &args[32..40];
    // convert locked_until to Since
    if !has_lock_time_passed(locked_until) {
        return Err(Error::LockTimeNotPassed);
    }

    Ok(())
}

pub fn has_lock_time_passed(locked_until: &[u8]) -> bool {
    let locked_until = Since::new(u64::from_le_bytes(locked_until.try_into().unwrap()));
    // all cell inputs must have a since value greater than locked_until
    for since in QueryIter::new(load_input_since, Source::Input) {
        let since = Since::new(since);
        if since.lt(&locked_until) {
            return false;
        }
    }
    true
}

pub fn required_lock_script_exists(required_lock_script_hash: &[u8]) -> bool {
    QueryIter::new(load_cell_lock_hash, Source::Input)
        .any(|cell_lock_hash| required_lock_script_hash[..] == cell_lock_hash[..])
}
