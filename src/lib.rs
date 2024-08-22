pub use balances::BalanceConfig;
pub use system::SystemConfig;
pub use vote::VoteConfig;

pub mod balances;
pub mod system;
pub mod vote;

pub struct Runtime;

// implement kiểu dữ liệu cụ thể  System cho runtime
impl SystemConfig for Runtime {
    type AccountId = u64;
}

// implement kiểu dữ liệu cụ thể  Balance cho runtime
impl BalanceConfig for Runtime {
    type Balance = u64;
}

// implement kiểu dữ liệu cụ thể  Vote cho runtime
impl VoteConfig for Runtime {}
