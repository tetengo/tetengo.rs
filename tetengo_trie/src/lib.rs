#![doc = include_str!("../README.md")]
#![doc = "# Examples"]
#![doc = "```rust"]
#![doc = include_str!("../tests/usage.rs")]
#![doc = "```"]

pub mod error;
pub mod file_mapping;
pub mod integer_serializer;
pub mod memory_storage;
pub mod mmap_storage;
pub mod serializer;
pub mod shared_storage;
pub mod storage;
pub mod string_serializer;
pub mod trie;
pub mod trie_iterator;
pub mod value_serializer;

mod double_array;
mod double_array_builder;
mod double_array_iterator;

pub use error::Error;
pub use file_mapping::FileMapping;
pub use integer_serializer::{IntegerDeserializer, IntegerSerializer};
pub use memory_storage::MemoryStorage;
pub use mmap_storage::MmapStorage;
pub use serializer::{Deserializer, DeserializerOf, Serializer, SerializerOf};
pub use shared_storage::SharedStorage;
pub use storage::Storage;
pub use string_serializer::{StrSerializer, StringDeserializer, StringSerializer};
pub use trie::{BuldingObserverSet, Trie};
pub use trie_iterator::TrieIterator;
pub use value_serializer::{ValueDeserializer, ValueSerializer};
