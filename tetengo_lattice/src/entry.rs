/*!
 * An entry.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::input::Input;

/**
 * A middle entry.
 */
#[derive(Debug)]
pub struct Middle {
    key: Rc<dyn Input>,
    value: Rc<dyn Any>,
    cost: i32,
}

impl Clone for Middle {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
            cost: self.cost,
        }
    }
}

/**
 * An entry.
 */
#[derive(Clone, Debug)]
pub enum Entry {
    /// The BOS/EOS (Beginning/Ending of Sequence) entry.
    BosEos,

    /// The middle entry.
    Middle(Middle),
}

impl Entry {
    /**
     * Creates an entry.
     *
     * # Arguments
     * * `key`   - A box of a key.
     * * `value` - A box of a value.
     * * `cost`  - A cost.
     */
    pub fn new(key: Box<dyn Input>, value: Box<dyn Any>, cost: i32) -> Self {
        Entry::Middle(Middle {
            key: Rc::from(key),
            value: Rc::from(value),
            cost,
        })
    }

    pub(crate) fn is_bos_eos(&self) -> bool {
        match self {
            Entry::BosEos => true,
            Entry::Middle(_) => false,
        }
    }

    /**
     * Returns the key.
     *
     * # Returns
     * The key.
     */
    pub fn key(&self) -> Option<&dyn Input> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.key.as_ref()),
        }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub fn value(&self) -> Option<&dyn Any> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.value.as_ref()),
        }
    }

    /**
     * Returns the cost.
     *
     * # Returns
     * The cost.
     */
    pub const fn cost(&self) -> i32 {
        match self {
            Entry::BosEos => 0,
            Entry::Middle(entry) => entry.cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_input::StringInput;

    use super::*;

    #[test]
    fn bos_eos() {
        let bos_eos = Entry::BosEos;

        assert!(bos_eos.key().is_none());
        assert!(bos_eos.value().is_none());
        assert_eq!(bos_eos.cost(), 0);
    }

    #[test]
    fn new() {
        let _entry = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );
    }

    #[test]
    fn clone() {
        let entry1 = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );
        let entry2 = entry1.clone();

        assert_eq!(
            entry1.key().unwrap().downcast_ref::<StringInput>(),
            entry2.key().unwrap().downcast_ref::<StringInput>()
        );
        assert_eq!(
            entry1.value().unwrap().downcast_ref::<String>(),
            entry2.value().unwrap().downcast_ref::<String>()
        );
        assert_eq!(entry1.cost(), entry2.cost());
    }

    #[test]
    fn key() {
        let entry = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );

        assert!(entry.key().is_some());
        assert!(entry.key().unwrap().is::<StringInput>());
        assert_eq!(
            entry
                .key()
                .unwrap()
                .downcast_ref::<StringInput>()
                .unwrap()
                .value(),
            "みずほ"
        );
    }

    #[test]
    fn value() {
        let entry = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );

        assert!(entry.value().is_some());
        assert!(entry.value().unwrap().is::<String>());
        assert_eq!(
            entry.value().unwrap().downcast_ref::<String>().unwrap(),
            "瑞穂"
        );
    }

    #[test]
    fn cost() {
        let entry = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );

        assert_eq!(entry.cost(), 42);
    }
}
