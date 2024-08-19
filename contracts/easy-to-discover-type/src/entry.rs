use alloc::vec::Vec;
use ckb_std::{ckb_types::prelude::Unpack, high_level::load_script};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::InsufficientArgsLength);
    }

    Ok(())
}
