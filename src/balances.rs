use std::collections::HashMap;
use std::result::Result;

use num::traits::{CheckedAdd, CheckedSub, Zero};

use crate::system::SystemConfig;

pub trait BalanceConfig: SystemConfig {
    // Định nghĩa kiểu dữ liệu Balance có khả năng thực hiện phép tính +/-
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

pub struct BalancePallet<T: BalanceConfig> {
    pub balances: HashMap<T::AccountId, T::Balance>,
}

impl<T: BalanceConfig> BalancePallet<T> {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    // set coins cho 1 account
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances.insert(who, amount);
    }

    // transfer coins
    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let fromBalance = self.get_balance(from.clone());
        let toBalance = self.get_balance(to.clone());
        self.set_balance(from, fromBalance - amount);
        self.set_balance(to, toBalance + amount);
        Ok(())
    }

    pub fn get_balance(&self, who: T::AccountId) -> T::Balance {
        *self.balances.get(&who).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::Runtime;

    use super::*;
    #[test]
    fn test_set_balance_should_work() {
        let alice = 1u64;
        let mut balance = BalancePallet::<Runtime>::new();
        //set balance
        balance.set_balance(alice, 1000);

        // check balance
        assert_eq!(balance.get_balance(alice), 1000u64);
    }

    #[test]
    fn test_set_transfer_should_work() {
        let alice = 1u64;
        let bob = 2u64;
        let mut balance = BalancePallet::<Runtime>::new();
        //set balance
        balance.set_balance(alice, 1000);

        // check balance alice before transfer
        assert_eq!(balance.get_balance(alice), 1000u64);

        // check balance bob before transfer
        assert_eq!(balance.get_balance(bob), 0u64);

        // Thực hiện transfer

        let result = balance.transfer(alice, bob, 100);

        assert!(result.is_ok());

        // check balance alice after transfer
        assert_eq!(balance.get_balance(alice), 900u64);

        // check balance bob after transfer
        assert_eq!(balance.get_balance(bob), 100u64);
    }
}
