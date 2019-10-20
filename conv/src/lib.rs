//! @brief Example Rust-based BPF program that prints out the parameters passed to it

#![allow(unreachable_code)]

extern crate solana_sdk;
use solana_sdk::{
    account_info::AccountInfo, entrypoint, entrypoint::SUCCESS, info, log::*, pubkey::Pubkey,
};

#[derive(Debug, PartialEq)]
struct SStruct {
    x: u64,
    y: u64,
    z: u64,
}

#[inline(never)]
fn return_sstruct() -> SStruct {
    SStruct { x: 1, y: 2, z: 3 }
}

entrypoint!(process_instruction);
fn process_instruction(program_id: &Pubkey, accounts: &mut [AccountInfo], data: &mut [u8]) -> u32 {
    info!("Program identifier:");
    program_id.log();

    // Log the provided account keys and instruction input data.  In the case of
    // the no-op program, no account keys or input data are expected but real
    // programs will have specific requirements so they can do their work.
    info!("Account keys and instruction input data:");
    sol_log_params(accounts, data);


    let (x, y): (u32, u32) = (deserialize_int32(&data[0..3]), deserialize_int32(&data[4..7]));
    let (img_start, img_end): (usize, usize) = (8 as usize, (8 + x*y*3 - 1) as usize);
    let (cx, cy): (u32, u32) = (deserialize_int32(&data[img_end..((img_end+3) as usize)]), deserialize_int32(&data[((img_end+4) as usize)..((img_end+7) as usize)]));
    let (fil_start, fil_end): (usize, usize) = ((img_end+8 as usize) as usize, ( img_end+8 as usize+ (cx*cy- 1)as usize) as usize);
    let result: usize = img_start;

    for i in 0..(x-cx) {
        for j in 0..(y-cy) {
            let (tx, ty, tz): (u32, u32, u32) = (0, 0, 0);
            for k in 0..cx{
                for l in 0..cy{
                    tx += (data[(i*y*3 + j*3) as usize] as u32 * data[(k*cy+l) as usize] as u32) as u32;
                    ty += (data[(i*y*3 + j*3 + 1) as usize] as u32 * data[(k*cy+l) as usize] as u32) as u32;
                    tz += (data[(i*y*3 + j*3 + 2) as usize] as u32 * data[(k*cy+l) as usize] as u32) as u32;
                }
            }
            data[result] = if(tx>255){255 as u8} else {if tx > 0 {tx as u8} else {0}};

            data[(result + 1) as usize] = if(ty>255){255 as u8} else {if ty > 0 {ty as u8} else {0}};

            data[(result + 2) as usize] = if(tz>255){255 as u8} else {if tz > 0 {tz as u8} else {0}};
            result += 3 as usize;

        }
    }

    info!("Filter Successfully Taken");
    SUCCESS
}

fn deserialize_int32(data: &[u8]) -> u32{
    let mut a: u32 = 0;
    for i in 0..3 {
        let x: u32 = (data[i] as u32)<< ((2 * i) as u32);
        a = a + x;
    }
    return a;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_return_sstruct() {
        assert_eq!(SStruct { x: 1, y: 2, z: 3 }, return_sstruct());
    }

}
