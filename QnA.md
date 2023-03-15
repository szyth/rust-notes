- Convert String into Int
```rust
let count: u32 = "42".parse().expect("Not a number!");

// OR
let count: i32 = match "d".parse() {
	Ok(num) => num,
	Err(_) => -1,
};
```
- what is `Option<T>` with examples?