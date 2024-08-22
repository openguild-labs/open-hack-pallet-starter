use std::collections::HashMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

use crate::system::SystemConfig;

pub trait BalanceConfig: SystemConfig {
    // Định nghĩa kiểu dữ liệu Balance có khả năng thực hiện phép tính +/- và so sánh
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + PartialEq;
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
        self.balances.insert(
            who.clone(),
            self.get_balance(who).checked_add(&amount).unwrap(),
        );
    }

    // transfer coins
    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        // check amount > 0
        if amount.is_zero() {
            return Err("Amount must be greater than 0");
        };

        let new_from_balance = self
            .get_balance(from.clone())
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        self.balances.insert(from.clone(), new_from_balance);

        let new_to_balance = self
            .get_balance(to.clone())
            .checked_add(&amount)
            .ok_or("Overflow balance")?;

        self.balances.insert(to.clone(), new_to_balance);
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

    #[test]
    fn test_set_transfer_should_return_error_insufficient_balance() {
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

        let result = balance.transfer(alice, bob, 100000);

        assert!(result.is_err());

        // check balance alice after transfer
        assert_eq!(balance.get_balance(alice), 1000u64);

        // check balance bob after transfer
        assert_eq!(balance.get_balance(bob), 0u64);
    }
}
