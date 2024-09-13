use std::hash::Hash;

pub trait SystemConfig {
	// Định nghĩa AccountId
	type AccountId: Eq + Hash + Clone;
}
