# Open Hack Pallet Starter - Polkadot Hackcamp Việt Nam 🇻🇳
Tutorial to guide developers about basic Pallet programming



## Coding challenge 1 cho chương trình Polkadot Bootcamp 2024
Qua 2 phần đầu tiên `Introduction to Substrate` và `Overview Polkadot SDK`, các bạn cũng nghe nhiều từ khoá `FRAME`, `Runtime` và `Pallet` . Phần coding này mô phỏng cơ bản `FRAME Pallet` và `Runtime` như thế nào dựa trên Polkadot SDK

Tóm gọn lại kiến thức:
+ `FRAME-pallet` : thư viện để build substrate runtime gồm có core frame (frame_support, frame_system, frame_executive), pallet chức năng , pallet parachain

+ `Runtime` : tập hợp các `FRAME-pallet`s tạo thành nhiều blockchain logic khác nhau


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

### Cách thức nộp bài
1. Step 1: `Fork` repo
2. Step 2: Tạo PR gồm có
+ Tên Discord
+ Link github mà bạn implement
+ Screenshot kết quả chạy `cargo test`