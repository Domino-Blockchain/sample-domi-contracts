// TODO: refactor domichain_program crate to allow WASM target
// TODO: use `core::alloc` crate in WASM

// #[macro_export]
// macro_rules! msg {
//     ($msg:expr) => {
//         sol_log($msg)
//     };
//     ($($arg:tt)*) => (sol_log(&format!($($arg)*)));
// }

// /// Print a string to the log.
// #[inline]
// pub fn sol_log(message: &str) {
//     // #[cfg(target_arch = "bpf")]
//     unsafe {
//         sol_log_(message.as_ptr(), message.len() as u64);
//     }

//     // #[cfg(not(target_arch = "bpf"))]
//     // crate::program_stubs::sol_log(message);
// }

// // #[cfg(target_arch = "bpf")]
// extern "C" {
//     fn sol_log_(message: *const u8, len: u64);
// }

use {
    // crate::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
    // alloc::vec::Vec,
    std::{
        // alloc::Layout,
        cell::RefCell,
        mem::size_of,
        // ptr::null_mut,
        rc::Rc,
        // result::Result as ResultGeneric,
        slice::{from_raw_parts, from_raw_parts_mut},
    },
};

use {
    // crate::{decode_error::DecodeError, instruction::InstructionError, msg, pubkey::PubkeyError},
    borsh::maybestd::io::Error as BorshIoError,
    // num_traits::{FromPrimitive, ToPrimitive},
    // std::convert::TryFrom,
    // thiserror::Error,
};
use borsh::{BorshDeserialize, BorshSerialize};



pub type Epoch = u64;

#[repr(transparent)]
#[derive(
    // AbiExample,
    // BorshDeserialize,
    // BorshSchema,
    // BorshSerialize,
    Clone,
    Copy,
    Default,
    Debug,
    // Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    // Pod,
    // Serialize,
    // Zeroable,
)]
pub struct Pubkey(pub(crate) [u8; 32]);

// use std::rc::Rc;

// use std::cell::RefCell;
// use core::cell::RefCell;

#[derive(Clone, Debug)]
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
    /// Was the transaction signed by this account's public key?
    pub is_signer: bool,
    /// Is the account writable?
    pub is_writable: bool,
    /// The lamports in the account.  Modifiable by programs.
    pub lamports: Rc<RefCell<&'a mut u64>>,
    /// The data held in this account.  Modifiable by programs.
    pub data: Rc<RefCell<&'a mut [u8]>>,
    /// Program that owns this account
    pub owner: &'a Pubkey,
    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// The epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
}

/// Reasons the program may fail
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgramError {
    /// Allows on-chain programs to implement program-specific error types and see them returned
    /// by the Solana runtime. A program-specific error may be any type that is represented as
    /// or serialized to a u32 integer.
    Custom(u32),
    InvalidArgument,
    InvalidInstructionData,
    InvalidAccountData,
    AccountDataTooSmall,
    InsufficientFunds,
    IncorrectProgramId,
    MissingRequiredSignature,
    AccountAlreadyInitialized,
    UninitializedAccount,
    NotEnoughAccountKeys,
    AccountBorrowFailed,
    MaxSeedLengthExceeded,
    InvalidSeeds,
    BorshIoError(String),
    AccountNotRentExempt,
    UnsupportedSysvar,
    IllegalOwner,
    MaxAccountsDataSizeExceeded,
    InvalidRealloc,
}

/// Builtin return values occupy the upper 32 bits
const BUILTIN_BIT_SHIFT: usize = 32;
macro_rules! to_builtin {
    ($error:expr) => {
        ($error as u64) << BUILTIN_BIT_SHIFT
    };
}

pub const CUSTOM_ZERO: u64 = to_builtin!(1);
pub const INVALID_ARGUMENT: u64 = to_builtin!(2);
pub const INVALID_INSTRUCTION_DATA: u64 = to_builtin!(3);
pub const INVALID_ACCOUNT_DATA: u64 = to_builtin!(4);
pub const ACCOUNT_DATA_TOO_SMALL: u64 = to_builtin!(5);
pub const INSUFFICIENT_FUNDS: u64 = to_builtin!(6);
pub const INCORRECT_PROGRAM_ID: u64 = to_builtin!(7);
pub const MISSING_REQUIRED_SIGNATURES: u64 = to_builtin!(8);
pub const ACCOUNT_ALREADY_INITIALIZED: u64 = to_builtin!(9);
pub const UNINITIALIZED_ACCOUNT: u64 = to_builtin!(10);
pub const NOT_ENOUGH_ACCOUNT_KEYS: u64 = to_builtin!(11);
pub const ACCOUNT_BORROW_FAILED: u64 = to_builtin!(12);
pub const MAX_SEED_LENGTH_EXCEEDED: u64 = to_builtin!(13);
pub const INVALID_SEEDS: u64 = to_builtin!(14);
pub const BORSH_IO_ERROR: u64 = to_builtin!(15);
pub const ACCOUNT_NOT_RENT_EXEMPT: u64 = to_builtin!(16);
pub const UNSUPPORTED_SYSVAR: u64 = to_builtin!(17);
pub const ILLEGAL_OWNER: u64 = to_builtin!(18);
pub const MAX_ACCOUNTS_DATA_SIZE_EXCEEDED: u64 = to_builtin!(19);
pub const INVALID_ACCOUNT_DATA_REALLOC: u64 = to_builtin!(20);
// Warning: Any new program errors added here must also be:
// - Added to the below conversions
// - Added as an equivilent to InstructionError
// - Be featureized in the BPF loader to return `InstructionError::InvalidError`
//   until the feature is activated

impl From<ProgramError> for u64 {
    fn from(error: ProgramError) -> Self {
        match error {
            ProgramError::InvalidArgument => INVALID_ARGUMENT,
            ProgramError::InvalidInstructionData => INVALID_INSTRUCTION_DATA,
            ProgramError::InvalidAccountData => INVALID_ACCOUNT_DATA,
            ProgramError::AccountDataTooSmall => ACCOUNT_DATA_TOO_SMALL,
            ProgramError::InsufficientFunds => INSUFFICIENT_FUNDS,
            ProgramError::IncorrectProgramId => INCORRECT_PROGRAM_ID,
            ProgramError::MissingRequiredSignature => MISSING_REQUIRED_SIGNATURES,
            ProgramError::AccountAlreadyInitialized => ACCOUNT_ALREADY_INITIALIZED,
            ProgramError::UninitializedAccount => UNINITIALIZED_ACCOUNT,
            ProgramError::NotEnoughAccountKeys => NOT_ENOUGH_ACCOUNT_KEYS,
            ProgramError::AccountBorrowFailed => ACCOUNT_BORROW_FAILED,
            ProgramError::MaxSeedLengthExceeded => MAX_SEED_LENGTH_EXCEEDED,
            ProgramError::InvalidSeeds => INVALID_SEEDS,
            ProgramError::BorshIoError(_) => BORSH_IO_ERROR,
            ProgramError::AccountNotRentExempt => ACCOUNT_NOT_RENT_EXEMPT,
            ProgramError::UnsupportedSysvar => UNSUPPORTED_SYSVAR,
            ProgramError::IllegalOwner => ILLEGAL_OWNER,
            ProgramError::MaxAccountsDataSizeExceeded => MAX_ACCOUNTS_DATA_SIZE_EXCEEDED,
            ProgramError::InvalidRealloc => INVALID_ACCOUNT_DATA_REALLOC,
            ProgramError::Custom(error) => {
                if error == 0 {
                    CUSTOM_ZERO
                } else {
                    error as u64
                }
            }
        }
    }
}

pub type ProgramResult = Result<(), ProgramError>;

impl From<u64> for ProgramError {
    fn from(error: u64) -> Self {
        match error {
            CUSTOM_ZERO => Self::Custom(0),
            INVALID_ARGUMENT => Self::InvalidArgument,
            INVALID_INSTRUCTION_DATA => Self::InvalidInstructionData,
            INVALID_ACCOUNT_DATA => Self::InvalidAccountData,
            ACCOUNT_DATA_TOO_SMALL => Self::AccountDataTooSmall,
            INSUFFICIENT_FUNDS => Self::InsufficientFunds,
            INCORRECT_PROGRAM_ID => Self::IncorrectProgramId,
            MISSING_REQUIRED_SIGNATURES => Self::MissingRequiredSignature,
            ACCOUNT_ALREADY_INITIALIZED => Self::AccountAlreadyInitialized,
            UNINITIALIZED_ACCOUNT => Self::UninitializedAccount,
            NOT_ENOUGH_ACCOUNT_KEYS => Self::NotEnoughAccountKeys,
            ACCOUNT_BORROW_FAILED => Self::AccountBorrowFailed,
            MAX_SEED_LENGTH_EXCEEDED => Self::MaxSeedLengthExceeded,
            INVALID_SEEDS => Self::InvalidSeeds,
            BORSH_IO_ERROR => Self::BorshIoError("Unknown".to_string()),
            ACCOUNT_NOT_RENT_EXEMPT => Self::AccountNotRentExempt,
            UNSUPPORTED_SYSVAR => Self::UnsupportedSysvar,
            ILLEGAL_OWNER => Self::IllegalOwner,
            MAX_ACCOUNTS_DATA_SIZE_EXCEEDED => Self::MaxAccountsDataSizeExceeded,
            INVALID_ACCOUNT_DATA_REALLOC => Self::InvalidRealloc,
            _ => Self::Custom(error as u32),
        }
    }
}

// impl TryFrom<InstructionError> for ProgramError {
//     type Error = InstructionError;

//     fn try_from(error: InstructionError) -> Result<Self, Self::Error> {
//         match error {
//             Self::Error::Custom(err) => Ok(Self::Custom(err)),
//             Self::Error::InvalidArgument => Ok(Self::InvalidArgument),
//             Self::Error::InvalidInstructionData => Ok(Self::InvalidInstructionData),
//             Self::Error::InvalidAccountData => Ok(Self::InvalidAccountData),
//             Self::Error::AccountDataTooSmall => Ok(Self::AccountDataTooSmall),
//             Self::Error::InsufficientFunds => Ok(Self::InsufficientFunds),
//             Self::Error::IncorrectProgramId => Ok(Self::IncorrectProgramId),
//             Self::Error::MissingRequiredSignature => Ok(Self::MissingRequiredSignature),
//             Self::Error::AccountAlreadyInitialized => Ok(Self::AccountAlreadyInitialized),
//             Self::Error::UninitializedAccount => Ok(Self::UninitializedAccount),
//             Self::Error::NotEnoughAccountKeys => Ok(Self::NotEnoughAccountKeys),
//             Self::Error::AccountBorrowFailed => Ok(Self::AccountBorrowFailed),
//             Self::Error::MaxSeedLengthExceeded => Ok(Self::MaxSeedLengthExceeded),
//             Self::Error::InvalidSeeds => Ok(Self::InvalidSeeds),
//             Self::Error::BorshIoError(err) => Ok(Self::BorshIoError(err)),
//             Self::Error::AccountNotRentExempt => Ok(Self::AccountNotRentExempt),
//             Self::Error::UnsupportedSysvar => Ok(Self::UnsupportedSysvar),
//             Self::Error::IllegalOwner => Ok(Self::IllegalOwner),
//             Self::Error::MaxAccountsDataSizeExceeded => Ok(Self::MaxAccountsDataSizeExceeded),
//             Self::Error::InvalidRealloc => Ok(Self::InvalidRealloc),
//             _ => Err(error),
//         }
//     }
// }

// impl<T> From<T> for InstructionError
// where
//     T: ToPrimitive,
// {
//     fn from(error: T) -> Self {
//         let error = error.to_u64().unwrap_or(0xbad_c0de);
//         match error {
//             CUSTOM_ZERO => Self::Custom(0),
//             INVALID_ARGUMENT => Self::InvalidArgument,
//             INVALID_INSTRUCTION_DATA => Self::InvalidInstructionData,
//             INVALID_ACCOUNT_DATA => Self::InvalidAccountData,
//             ACCOUNT_DATA_TOO_SMALL => Self::AccountDataTooSmall,
//             INSUFFICIENT_FUNDS => Self::InsufficientFunds,
//             INCORRECT_PROGRAM_ID => Self::IncorrectProgramId,
//             MISSING_REQUIRED_SIGNATURES => Self::MissingRequiredSignature,
//             ACCOUNT_ALREADY_INITIALIZED => Self::AccountAlreadyInitialized,
//             UNINITIALIZED_ACCOUNT => Self::UninitializedAccount,
//             NOT_ENOUGH_ACCOUNT_KEYS => Self::NotEnoughAccountKeys,
//             ACCOUNT_BORROW_FAILED => Self::AccountBorrowFailed,
//             MAX_SEED_LENGTH_EXCEEDED => Self::MaxSeedLengthExceeded,
//             INVALID_SEEDS => Self::InvalidSeeds,
//             BORSH_IO_ERROR => Self::BorshIoError("Unknown".to_string()),
//             ACCOUNT_NOT_RENT_EXEMPT => Self::AccountNotRentExempt,
//             UNSUPPORTED_SYSVAR => Self::UnsupportedSysvar,
//             ILLEGAL_OWNER => Self::IllegalOwner,
//             MAX_ACCOUNTS_DATA_SIZE_EXCEEDED => Self::MaxAccountsDataSizeExceeded,
//             INVALID_ACCOUNT_DATA_REALLOC => Self::InvalidRealloc,
//             _ => {
//                 // A valid custom error has no bits set in the upper 32
//                 if error >> BUILTIN_BIT_SHIFT == 0 {
//                     Self::Custom(error as u32)
//                 } else {
//                     Self::InvalidError
//                 }
//             }
//         }
//     }
// }

// impl From<PubkeyError> for ProgramError {
//     fn from(error: PubkeyError) -> Self {
//         match error {
//             PubkeyError::MaxSeedLengthExceeded => Self::MaxSeedLengthExceeded,
//             PubkeyError::InvalidSeeds => Self::InvalidSeeds,
//             PubkeyError::IllegalOwner => Self::IllegalOwner,
//         }
//     }
// }

impl From<BorshIoError> for ProgramError {
    fn from(error: BorshIoError) -> Self {
        Self::BorshIoError(format!("{}", error))
    }
}



pub unsafe fn deserialize<'a>(input: *mut u8) -> (&'a Pubkey, Vec<AccountInfo<'a>>, &'a [u8]) {
    let mut offset: usize = 0;

    // Number of accounts present

    #[allow(clippy::cast_ptr_alignment)]
    let num_accounts = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    // Account Infos

    let mut accounts = Vec::with_capacity(num_accounts);
    for _ in 0..num_accounts {
        let dup_info = *(input.add(offset) as *const u8);
        offset += size_of::<u8>();
        if dup_info == std::u8::MAX {
            #[allow(clippy::cast_ptr_alignment)]
            let is_signer = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            #[allow(clippy::cast_ptr_alignment)]
            let is_writable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            #[allow(clippy::cast_ptr_alignment)]
            let executable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            // The original data length is stored here because these 4 bytes were
            // originally only used for padding and served as a good location to
            // track the original size of the account data in a compatible way.
            let original_data_len_offset = offset;
            offset += size_of::<u32>();

            let key: &Pubkey = &*(input.add(offset) as *const Pubkey);
            offset += size_of::<Pubkey>();

            let owner: &Pubkey = &*(input.add(offset) as *const Pubkey);
            offset += size_of::<Pubkey>();

            #[allow(clippy::cast_ptr_alignment)]
            let lamports = Rc::new(RefCell::new(&mut *(input.add(offset) as *mut u64)));
            offset += size_of::<u64>();

            #[allow(clippy::cast_ptr_alignment)]
            let data_len = *(input.add(offset) as *const u64) as usize;
            offset += size_of::<u64>();

            // Store the original data length for detecting invalid reallocations and
            // requires that MAX_PERMITTED_DATA_LENGTH fits in a u32
            *(input.add(original_data_len_offset) as *mut u32) = data_len as u32;

            let data = Rc::new(RefCell::new({
                from_raw_parts_mut(input.add(offset), data_len)
            }));
            offset += data_len + MAX_PERMITTED_DATA_INCREASE;
            offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128); // padding

            #[allow(clippy::cast_ptr_alignment)]
            let rent_epoch = *(input.add(offset) as *const u64);
            offset += size_of::<u64>();

            accounts.push(AccountInfo {
                key,
                is_signer,
                is_writable,
                lamports,
                data,
                owner,
                executable,
                rent_epoch,
            });
        } else {
            offset += 7; // padding

            // Duplicate account, clone the original
            accounts.push(accounts[dup_info as usize].clone());
        }
    }

    // Instruction data

    #[allow(clippy::cast_ptr_alignment)]
    let instruction_data_len = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    let instruction_data = { from_raw_parts(input.add(offset), instruction_data_len) };
    offset += instruction_data_len;

    // Program Id

    let program_id: &Pubkey = &*(input.add(offset) as *const Pubkey);

    (program_id, accounts, instruction_data)
}

/// Maximum number of bytes a program may add to an account during a single realloc
pub const MAX_PERMITTED_DATA_INCREASE: usize = 1_024 * 10;

/// `assert_eq(std::mem::align_of::<u128>(), 8)` is true for BPF but not for some host machines
pub const BPF_ALIGN_OF_U128: usize = 8;


pub fn next_account_info<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> Result<I::Item, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}


/// Print a string to the log.
#[inline]
pub fn sol_log(message: &str) {
    // #[cfg(target_arch = "bpf")]
    unsafe {
        sol_log_(message.as_ptr(), message.len() as u64);
    }
}


#[macro_export]
macro_rules! msg {
    ($msg:expr) => {
        $crate::domichain_program::sol_log($msg)
    };
    ($($arg:tt)*) => ($crate::domichain_program::sol_log(&format!($($arg)*)));
}

#[macro_export]
macro_rules! stack_msg {
    ($msg:expr) => {
        msg!($msg)
    };
    ($($arg:tt)*) => {
        let mut buf = [0u8; 256];
        let s: &str = write_to::show(
            &mut buf,
            format_args!($($arg)*),
        ).unwrap();
        msg!(s);
    };
}

// #[cfg(target_arch = "bpf")]
extern "C" {
    fn sol_log_(message: *const u8, len: u64);

    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;

    fn sol_keccak256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

/// Return a Keccak256 hash for the given data.
pub fn hashv(vals: &[&[u8]]) -> Hash {
    // Call via a system call to perform the calculation
    let mut hash_result = [0; HASH_BYTES];
    unsafe {
        sol_keccak256(
            vals as *const _ as *const u8,
            vals.len() as u64,
            &mut hash_result as *mut _ as *mut u8,
        );
    }
    Hash::new_from_array(hash_result)
}

/// Return a Keccak256 hash for the given data.
pub fn hash(val: &[u8]) -> Hash {
    hashv(&[val])
}

pub const HASH_BYTES: usize = 32;
/// Maximum string length of a base58 encoded hash
const MAX_BASE58_LEN: usize = 44;
#[derive(
    BorshSerialize,
    BorshDeserialize,
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[repr(transparent)]
pub struct Hash(pub [u8; HASH_BYTES]);

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

use std::fmt;
impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl Hash {
    pub fn new(hash_slice: &[u8]) -> Self {
        Hash(<[u8; HASH_BYTES]>::try_from(hash_slice).unwrap())
    }

    pub const fn new_from_array(hash_array: [u8; HASH_BYTES]) -> Self {
        Self(hash_array)
    }
}


/// Programs indicate success with a return value of 0
pub const SUCCESS: u64 = 0;

/// Start address of the memory region used for program heap.
pub const HEAP_START_ADDRESS: u64 = 8 * 1024;
// pub const HEAP_START_ADDRESS: u64 = 0x300000000;
/// Length of the heap memory region used for program heap.
pub const HEAP_LENGTH: usize = 16 * 1024;
// pub const HEAP_LENGTH: usize = 32 * 1024;

/// The bump allocator used as the default rust heap when running programs.
pub struct BumpAllocator {
    pub start: usize,
    pub len: usize,
}
/// Integer arithmetic in this global allocator implementation is safe when
/// operating on the prescribed `HEAP_START_ADDRESS` and `HEAP_LENGTH`. Any
/// other use may overflow and is thus unsupported and at one's own risk.
#[allow(clippy::integer_arithmetic)]
unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let pos_ptr = self.start as *mut usize;

        let mut pos = *pos_ptr;
        if pos == 0 {
            // First time, set starting position
            pos = self.start + self.len;
        }
        pos = pos.saturating_sub(layout.size());
        pos &= !(layout.align().wrapping_sub(1));
        if pos < self.start + core::mem::size_of::<*mut u8>() {
            return core::ptr::null_mut();
        }
        *pos_ptr = pos;
        
        let res = pos as *mut u8;
        res
    }
    #[inline]
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        // I'm a bump allocator, I don't free
    }
}

#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) =
                unsafe { $crate::domichain_program::deserialize(input) };
            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => $crate::domichain_program::SUCCESS,
                Err(error) => error.into(),
            }
        }
        // $crate::custom_heap_default!();
        // $crate::custom_panic_default!();
    };
}

// #[no_mangle]
// pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
//     let (program_id, accounts, instruction_data) =
//         unsafe { deserialize(input) };
//     match process_instruction(
//         &program_id, &accounts, &instruction_data
//     ) {
//         Ok(()) => SUCCESS,
//         Err(error) => error.into(),
//     }
// }

#[global_allocator]
static GLOBAL: BumpAllocator = BumpAllocator {
    start: HEAP_START_ADDRESS as usize,
    len: HEAP_LENGTH,
};