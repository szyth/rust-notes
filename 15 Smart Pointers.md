- A _pointer_ contains an address in memory.
- All Pointers are `usize`d
	- aka whole memory segment ( idk, what its called, a row? )
- The most common kind of pointer is a reference `&`
	- borrows the value they point to
	- Don’t have any special capabilities other than referring to data, and have no overhead.
- Size of Data on stack is known at Compile-time.
- Size of Data on heap is evaluated at Run-time only
---
[Raw Pointers](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/raw-pointers.html)

---
### Smart Pointers

- are Data structures
- acts like a Pointer
- have additional metadata and capabilities
- eg: _reference counting_ smart pointer
	- This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.
- while references only borrow data, in many cases, smart pointers _own_ the data they point to.
-   Standard library Smart pointers:
	- `Box<T>` for allocating values on the heap
	- `Rc<T>`, a reference counting type that enables multiple ownership
	- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time
- `String` and `Vec<T>` are actually Smart Pointers.
- usually implemented using structs but unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits
	- discussed in details in following chapter, along with:
