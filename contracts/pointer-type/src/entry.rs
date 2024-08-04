use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed::OutPoint,
        prelude::{Builder, Entity, Pack, Unpack},
    },
    high_level::{
        load_cell_data, load_cell_data_hash, load_cell_type, load_script, load_tx_hash, QueryIter,
    },
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Vec<u8> = script.args().unpack();
    if args.len() < 32 {
        return Err(Error::Encoding);
    }

    let code_or_type_hash = &args[..32];
    // pointer-type cannot be put in Inputs field, which means script deployed by `code_hash` is non-upgradable
    // backed by pointer-type script, by the way, to create a new pointer-type cell to achieve this upgrade demand
    let payload = load_cell_data(0, Source::GroupOutput).map_err(|_| Error::NotUpgradable)?;
    let Some(flag) = payload.first() else {
        return Err(Error::InvalidFlag);
    };

    match *flag {
        // point to code: use out_point to get preimage that under the code hash
        0 => {
            let (out_index, _) = QueryIter::new(load_cell_data_hash, Source::Output)
                .enumerate()
                .find(|(_, hash)| hash == code_or_type_hash)
                .ok_or(Error::DataHashNotMatch)?;
            let tx_hash = load_tx_hash()?;
            let out_point = OutPoint::new_builder()
                .tx_hash(tx_hash.pack())
                .index(out_index.pack())
                .build();
            if &payload[..OutPoint::TOTAL_SIZE] != out_point.as_slice() {
                return Err(Error::OutPointPayloadNotMatch);
            }
            Ok(())
        }
        // point to type: use type hash to get preimage that under the type hash
        1 => {
            let type_script = QueryIter::new(load_cell_type, Source::Output)
                .flatten()
                .find(|script| {
                    let hash = script.calc_script_hash().raw_data();
                    hash.as_ref() == code_or_type_hash
                })
                .ok_or(Error::TypeHashNotMatch)?;
            let script_bytes = type_script.as_slice();
            if &payload[..script_bytes.len()] != script_bytes {
                return Err(Error::TypeScriptPayloadNotMatch);
            }
            Ok(())
        }
        // bad pointer flag
        _ => Err(Error::InvalidFlag),
    }
}
