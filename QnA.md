- Convert String into Int
```rust
let count: u32 = "42".parse().expect("Not a number!");

// OR
let count: i32 = match "d".parse() {
	Ok(num) => num,
	Err(_) => -1,
};
```
- Convert Byte to Char
```rust
let byte = 97;
let char = format!("{}", byte as char)
```
- Difference between `Option<T>` and `Result<T,E>` ?
	- `Option` can just represent _that_ an operation has failed, but `Result` can explain (all the reasons) _why_  the operation has failed.
	- `Option<T>` returns `Some,None`. 
	- `Result<T,E>` returns `Ok,Err`
		- eg: `parse(), File::open(), read_to_string()`
- what is `Option<T>` with examples?
