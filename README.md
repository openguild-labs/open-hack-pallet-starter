# open-hack-pallet-starter
Tutorial to guide developers about basic Pallet programming



## Coding challenge 1 cho chương trình Polkadot Bootcamp 2024

### Mô phỏng đơn giản Pallet

Gồm có:
+ frame_system: `system.rs`
+ pallet_balances: `balances.rs`
+ pallet_vote: `vote.rs` 

### Mô phỏng Runtime

```rust
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
impl VoteConfig for Runtime {
    
}
```


### Nhiệm vụ 
+ Implement logic còn thiếu của module `balances.rs` và `vote.rs`
+ Kiểm tra logic đúng hay sai dựa trên câu lệnh `cargo test` 


### Nộp bài 
+ Link github mà bạn implement 
+ Screenshot kết quả chạy `cargo test` 








