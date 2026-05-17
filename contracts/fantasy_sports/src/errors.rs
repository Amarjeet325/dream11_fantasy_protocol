use soroban_sdk::contracterror;

/// Custom error codes for the fantasy sports smart contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// The provided entry fee is negative.
    EntryFeeNegative = 101,
    /// A contest with this ID already exists.
    ContestAlreadyExists = 102,
    /// The requested contest does not exist.
    ContestNotFound = 103,
    /// The contest is no longer active (e.g. finalized or cancelled).
    ContestNotActive = 104,
    /// The contest has already been finalized and a winner declared.
    ContestAlreadyFinalized = 105,
    /// The caller is not authorized to perform the action.
    Unauthorized = 106,
    /// Attempting to join a contest with duplicate entry.
    DuplicateEntry = 107,
}
