#![no_std]

mod domichain_program;

use domichain_program::*;

// TODO: deserialize input
#[no_mangle]
pub unsafe extern "C" fn entrypoint(_input: *mut u8) -> u64 {
    //     let program_id = Default::default();
    //     let mut _lamports = 0;
    //     let mut _data = [];
    //     let lamports = Rc::new(RefCell::new(&mut _lamports));
    //     let data = Rc::new(RefCell::new(&mut _data));
    //     let accounts = [AccountInfo {
    //         key: &program_id,
    //         /// Was the transaction signed by this account's public key?
    //         is_signer: false,
    //         /// Is the account writable?
    //         is_writable: false,
    //         /// The lamports in the account.  Modifiable by programs.
    //         lamports,
    //         /// The data held in this account.  Modifiable by programs.
    //         data,
    //         /// Program that owns this account
    //         owner: &program_id,
    //         /// This account's data contains a loaded program (and is now read-only)
    //         executable: false,
    //         /// The epoch at which this account will next owe rent
    //         rent_epoch: Default::default(),
    //     }];
    //     let instruction_data = [];
    match process_instruction() {
        Ok(()) => 0,
        Err(error) => error.into(),
    }
}

// TODO: use WASM allocator
// #[global_allocator]
// static A: solana_program::entrypoint::BumpAllocator = solana_program::entrypoint::BumpAllocator {
//     start: solana_program::entrypoint::HEAP_START_ADDRESS as usize,
//     len: solana_program::entrypoint::HEAP_LENGTH,
// };

// TODO: use WASM panic handler
#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn process_instruction(
    // _program_id: &Pubkey,
    // _accounts: &[AccountInfo],
    // _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from syscall!");
    Ok(())
}