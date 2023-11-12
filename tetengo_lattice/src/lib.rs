#![doc = include_str!("../README.md")]
//#![doc = "# Examples"]
//#![doc = "```rust"]
//#![doc = include_str!("../tests/usage.rs")]
//#![doc = "```"]

pub mod entry;
pub mod input;
pub mod string_input;

pub use entry::Entry;
pub use input::Input;
pub use input::InputError;
pub use string_input::StringInput;
