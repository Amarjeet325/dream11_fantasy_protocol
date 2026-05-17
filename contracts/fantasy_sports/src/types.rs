use soroban_sdk::{contracttype, Address, String};

/// Storage keys for the smart contract's state.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Stores the details of a contest. Keyed by the unique contest ID.
    Contest(u64),
}

/// Represents the structure of a fantasy sports contest on-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contest {
    /// The address of the creator/manager of the contest.
    pub creator: Address,
    /// The token used for entry fees and reward distribution (e.g., USDC, XLM wrapper).
    pub token: Address,
    /// The entry fee required to join the contest.
    pub entry_fee: i128,
    /// The total prize pool collected so far.
    pub prize_pool: i128,
    /// Metadata or external link (e.g., IPFS hash) containing team details, rules, etc.
    pub details: String,
    /// The address of the winner (populated once the contest is finalized).
    pub winner: Option<Address>,
    /// Status indicating whether the contest is still active and accepting entries.
    pub is_active: bool,
}
