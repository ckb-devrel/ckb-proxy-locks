use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed::OutPoint,
        prelude::{Entity, Unpack},
    },
    high_level::{load_input, load_script, QueryIter},
};

use crate::error::Error;

pub fn check_outpoint(outpoint: OutPoint) -> bool {
    let is_owner_mode = QueryIter::new(load_input, Source::Input).any(|cell_input| {
        if cell_input.previous_output() == outpoint {
            return true;
        }

        false
    });

    is_owner_mode
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < OutPoint::TOTAL_SIZE {
        return Err(Error::Encoding);
    }

    let owner_outpoint = OutPoint::from_slice(&args[0..OutPoint::TOTAL_SIZE])?;
    if !check_outpoint(owner_outpoint) {
        return Err(Error::OutpointNotFound);
    }

    Ok(())
}
