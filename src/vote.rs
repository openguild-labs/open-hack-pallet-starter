use std::collections::HashMap;

use crate::system::SystemConfig;

pub trait VoteConfig: SystemConfig {}

pub struct VotePallet<T: VoteConfig> {
    pub votes: HashMap<(T::AccountId, T::AccountId), bool>,
}

impl<T: VoteConfig> VotePallet<T> {
    pub fn new() -> Self {
        todo!()
    }

    // Vote Yes

    pub fn vote(&mut self, who: T::AccountId, voter: T::AccountId) -> Result<(), &'static str> {
        todo!()
    }

    // Vote No

    pub fn revoke(&mut self, who: T::AccountId, voter: T::AccountId) -> Result<(), &'static str> {
        todo!()
    }

    pub fn get_vote(&self, who: T::AccountId, voter: T::AccountId) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Runtime;

    use super::*;
    #[test]
    fn test_vote_should_work() {
        let alice = 1u64;
        let bob = 2u64;
        let mut vote = VotePallet::<Runtime>::new();

        // alice vote cho bob

        let result = vote.vote(alice, bob);
        assert!(result.is_ok());

        // kiểm tra vote
        let yes = vote.get_vote(alice, bob);
        assert_eq!(yes, true);

        // alice revoke bob

        let result = vote.revoke(alice, bob);
        assert!(result.is_ok());

        // kiểm tra vote
        let no = vote.get_vote(alice, bob);
        assert_eq!(no, false);
    }
}
