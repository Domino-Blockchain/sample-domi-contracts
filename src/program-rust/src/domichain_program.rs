// TODO: refactor domichain_program crate to allow WASM target
// TODO: use `core::alloc` crate in WASM

#[macro_export]
macro_rules! msg {
    ($msg:expr) => {
        sol_log($msg)
    };
    ($($arg:tt)*) => (sol_log(&format!($($arg)*)));
}

/// Print a string to the log.
#[inline]
pub fn sol_log(message: &str) {
    // #[cfg(target_arch = "bpf")]
    unsafe {
        sol_log_(message.as_ptr(), message.len() as u64);
    }

    // #[cfg(not(target_arch = "bpf"))]
    // crate::program_stubs::sol_log(message);
}

// #[cfg(target_arch = "bpf")]
extern "C" {
    fn sol_log_(message: *const u8, len: u64);
}


// pub type Epoch = u64;

// #[derive(Default)]
// pub struct Pubkey(pub(crate) [u8; 32]);

// #[derive(Clone)]
// pub struct AccountInfo<'a> {
//     /// Public key of the account
//     pub key: &'a Pubkey,
//     /// Was the transaction signed by this account's public key?
//     pub is_signer: bool,
//     /// Is the account writable?
//     pub is_writable: bool,
//     /// The lamports in the account.  Modifiable by programs.
//     pub lamports: Rc<RefCell<&'a mut u64>>,
//     /// The data held in this account.  Modifiable by programs.
//     pub data: Rc<RefCell<&'a mut [u8]>>,
//     /// Program that owns this account
//     pub owner: &'a Pubkey,
//     /// This account's data contains a loaded program (and is now read-only)
//     pub executable: bool,
//     /// The epoch at which this account will next owe rent
//     pub rent_epoch: Epoch,
// }

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
    // BorshIoError(String), // FIXME: use `alloc` crate
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
// pub const BORSH_IO_ERROR: u64 = to_builtin!(15);
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
            // ProgramError::BorshIoError(_) => BORSH_IO_ERROR, // FIXME: use `alloc` crate
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