/*!
 * A trie.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use crate::double_array::{DoubleArray, DEFAULT_DENSITY_FACTOR};
use crate::serializer::{Serializer, SerializerOf};

/**
 * A result type.
 *
 * # Type Parameters
 * * `T` - A type.
 */
pub type Result<T> = anyhow::Result<T>;

/// The default double array density factor.
const _DEFAULT_DOUBLE_ARRAY_DENSITY_FACTOR: usize = DEFAULT_DENSITY_FACTOR;

/**
 * A trie.
 */
#[derive(Debug)]
pub struct Trie<Key, Value, KeySerializer: Serializer = <() as SerializerOf<Key>>::Type> {
    _phantom: std::marker::PhantomData<Key>,
    _double_array: DoubleArray<Value>,
    _key_serializer: KeySerializer,
}

impl<Key, Value: Clone + 'static, KeySerializer: Serializer> Trie<Key, Value, KeySerializer> {
    /**
     * Creates a trie.
     */
    pub fn new() -> Result<Self> {
        Self::new_with_keyserializer(KeySerializer::new(true))
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `key_serializer` - A key serializer.
     */
    pub fn new_with_keyserializer(key_serializer: KeySerializer) -> Result<Self> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
            _double_array: DoubleArray::new()?,
            _key_serializer: key_serializer,
        })
    }

    /**
     * Creates a trie.
     *
     * # Arguments
     * * `elements` - Elements.
     */
    pub fn new_with_elements(elements: Vec<(KeySerializer::Object<'_>, Value)>) -> Result<Self> {
        Self::new_with_elements_keyserializer(elements, KeySerializer::new(true))
    }

    /**
     * Creates a trie with elements and a key serializer.
     *
     * # Arguments
     * * `elements`       - Elements.
     * * `key_serializer` - A key serializer.
     */
    pub fn new_with_elements_keyserializer(
        elements: Vec<(KeySerializer::Object<'_>, Value)>,
        key_serializer: KeySerializer,
    ) -> Result<Self> {
        let mut double_array_content_keys = Vec::<String>::with_capacity(elements.len());
        for element in &elements {
            let (key, _) = &element;
            let serialized_key = String::from_utf8(key_serializer.serialize(key))?;
            double_array_content_keys.push(serialized_key);
        }
        let mut double_array_contents = Vec::<(&str, i32)>::with_capacity(elements.len());
        for (i, _) in elements.iter().enumerate() {
            double_array_contents.push((&double_array_content_keys[i], i as i32));
        }

        //     const double_array::building_observer_set_type double_array_building_observer_set{
        //         [&building_observer_set](const std::pair<std::string_view, std::int32_t>& element) {
        //             building_observer_set.adding(element.first);
        //         },
        //         [&building_observer_set]() { building_observer_set.done(); }
        //     };

        let mut double_array = DoubleArray::<Value>::new_with_elements(double_array_contents)?;

        for (i, element) in elements.into_iter().enumerate() {
            let (_, value) = element;
            double_array.storage_mut().add_value_at(i, value)?;
        }

        Ok(Self {
            _phantom: std::marker::PhantomData,
            _double_array: double_array,
            _key_serializer: key_serializer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::string_serializer::StringSerializer;

    use super::*;

    const KUMAMOTO: &str = "熊本";

    static TAMANA: &str = "玉名";

    static _TAMARAI: &str = "玉来";

    static _TAMA: &str = "玉";

    static _UTO: &str = "宇土";

    #[test]
    fn test_new() {
        let _trie = Trie::<&str, i32>::new().unwrap();
    }

    #[test]
    fn new_with_keyserializer() {
        let key_serializer = StringSerializer::new(true);
        let _trie = Trie::<&str, i32>::new_with_keyserializer(key_serializer).unwrap();
    }

    #[test]
    fn new_with_elements() {
        {
            let _trie =
                Trie::<&str, i32>::new_with_elements([("Kumamoto", 42), ("Tamana", 24)].to_vec())
                    .unwrap();
        }
        {
            let _trie = Trie::<&str, String>::new_with_elements(
                [
                    (KUMAMOTO, KUMAMOTO.to_string()),
                    (TAMANA, TAMANA.to_string()),
                ]
                .to_vec(),
            )
            .unwrap();
        }
    }

    #[test]
    fn new_with_elements_keyserializer() {
        let content = [
            ("kumamoto", KUMAMOTO.to_string()),
            ("tamana", TAMANA.to_string()),
        ]
        .to_vec();
        let _trie = Trie::<&str, String>::new_with_elements_keyserializer(
            content,
            StringSerializer::new(true),
        )
        .unwrap();
    }
}
