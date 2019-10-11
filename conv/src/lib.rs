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
fn process_instruction(program_id: &Pubkey, accounts: &mut [AccountInfo], data: &[u8]) -> u32 {
    info!("Program identifier:");
    program_id.log();

    // Log the provided account keys and instruction input data.  In the case of
    // the no-op program, no account keys or input data are expected but real
    // programs will have specific requirements so they can do their work.
    info!("Account keys and instruction input data:");
    sol_log_params(accounts, data);

    info!("Taking input the size of Filter");
    let (cx, cy): (u32, u32) = (deserializeInt32(&data[0..3]), deserializeInt32(&data[4..7]));
    assert_eq!(cx, 2);
    assert_eq!(cy, 2);
    info!("Size successfully taken");
    let offset: usize = (4*2+cx*cy) as usize;
    let (x, y): (u32, u32) = (deserializeInt32(&data[offset..(offset+3)]), deserializeInt32(&data[(offset+4)..(offset+7)]));
    assert_eq!(x, 3);
    assert_eq!(y, 3);
    info!("Filter Successfully Taken");
    SUCCESS
}

fn deserializeInt32(data: &[u8]) -> u32{
    let mut a: u32 = 0;
    for i in 0..3 {
        let x: u32 = (data[i] << 2*i).into();
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
