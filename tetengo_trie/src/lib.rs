#![doc = include_str!("../README.md")]
#![doc = "# Examples"]
#![doc = "```rust"]
#![doc = include_str!("../tests/usage.rs")]
#![doc = "```"]

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

pub use integer_serializer::IntegerDeserializer;
pub use integer_serializer::IntegerSerializer;
pub use memory_storage::MemoryStorage;
pub use mmap_storage::FileMapping;
pub use mmap_storage::MmapStorage;
pub use serializer::DeserializationError;
pub use serializer::Deserializer;
pub use serializer::DeserializerOf;
pub use serializer::Serializer;
pub use serializer::SerializerOf;
pub use shared_storage::SharedStorage;
pub use storage::Storage;
pub use storage::StorageError;
pub use string_serializer::StrSerializer;
pub use string_serializer::StringDeserializer;
pub use trie::BuldingObserverSet;
pub use trie::Trie;
pub use trie_iterator::TrieIterator;
pub use value_serializer::ValueDeserializer;
pub use value_serializer::ValueSerializer;
