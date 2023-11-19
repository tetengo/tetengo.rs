/*!
 * An entry.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::{self, Debug, Formatter};

use crate::input::Input;

/**
 * An any value.
 */
pub trait AnyValue: Any {
    /**
     * Clones this object.
     *
     * # Returns
     * A box of a clone of this object.
     */
    fn clone_box(&self) -> Box<dyn AnyValue>;

    /**
     * Returns this object as 'Any'.
     *
     * # Returns
     * This object as 'Any'.
     */
    fn as_any(&self) -> &dyn Any;
}

impl<T: Clone + 'static> AnyValue for T {
    fn clone_box(&self) -> Box<dyn AnyValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/**
 * A middle entry.
 */
pub struct Middle {
    key: Box<dyn Input>,
    value: Box<dyn AnyValue>,
    cost: i32,
}

impl Debug for Middle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddleEntry")
            .field("key", &"Box<dyn Input>")
            .field("value", &"Box<dyn AnyValue>")
            .field("cost", &self.cost)
            .finish()
    }
}

impl Clone for Middle {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone_box(),
            value: self.value.clone_box(),
            cost: self.cost,
        }
    }
}

/**
 * An entry.
 */
#[derive(Debug, Clone)]
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
    pub fn new(key: Box<dyn Input>, value: Box<dyn AnyValue>, cost: i32) -> Self {
        Entry::Middle(Middle { key, value, cost })
    }

    /**
     * Creates an entry.
     *
     * # Arguments
     * * `view` - An entry view.
     */
    pub fn from_view(view: &EntryView<'_>) -> Self {
        match view {
            EntryView::BosEos => Entry::BosEos,
            EntryView::Middle(middle_view) => Entry::new(
                middle_view.key.clone_box(),
                middle_view.value.clone_box(),
                middle_view.cost,
            ),
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
    pub fn value(&self) -> Option<&dyn AnyValue> {
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
    pub fn cost(&self) -> i32 {
        match self {
            Entry::BosEos => 0,
            Entry::Middle(entry) => entry.cost,
        }
    }
}

/**
 * An middle entry view.
 */
#[derive(Clone)]
pub struct MiddleView<'a> {
    key: &'a dyn Input,
    value: &'a dyn AnyValue,
    cost: i32,
}

impl Debug for MiddleView<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddleEntryView")
            .field("key", &"Option<&dyn Input>")
            .field("value", &"Option<&'a dyn AnyValue>")
            .field("cost", &self.cost)
            .finish()
    }
}

/**
 * An entry view.
 */
#[derive(Debug, Clone)]
pub enum EntryView<'a> {
    /// The BOS/EOS (Beginning/Ending of Sequence) entry.
    BosEos,

    /// The middle entry.
    Middle(MiddleView<'a>),
}

impl<'a> EntryView<'a> {
    /**
     * Creates an entry view.
     *
     * # Arguments
     * * `key`   - A key.
     * * `value` - A value.
     * * `cost`  - A cost.
     */
    pub const fn new(key: &'a dyn Input, value: &'a dyn AnyValue, cost: i32) -> Self {
        EntryView::Middle(MiddleView { key, value, cost })
    }

    /**
     * Creates an entry view.
     *
     * # Arguments
     * * `entry` - An entry.
     */
    pub fn from_entry(entry: &'a Entry) -> Self {
        match entry {
            Entry::BosEos => EntryView::BosEos,
            Entry::Middle(middle) => {
                EntryView::new(middle.key.as_ref(), middle.value.as_ref(), middle.cost)
            }
        }
    }

    /**
     * Returns the key.
     *
     * # Returns
     * The key.
     */
    pub const fn key(&self) -> Option<&'a dyn Input> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.key),
        }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub const fn value(&self) -> Option<&'a dyn AnyValue> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.value),
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
            EntryView::BosEos => 0,
            EntryView::Middle(middle_view) => middle_view.cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StringInput;

    use super::*;

    #[test]
    fn bos_eos() {
        {
            let bos_eos = Entry::BosEos;

            assert!(bos_eos.key().is_none());
            assert!(bos_eos.value().is_none());
            assert_eq!(bos_eos.cost(), 0);
        }
        {
            let bos_eos = EntryView::BosEos;

            assert!(bos_eos.key().is_none());
            assert!(bos_eos.value().is_none());
            assert_eq!(bos_eos.cost(), 0);
        }
    }

    #[test]
    fn new() {
        {
            let _entry = Entry::new(
                Box::new(StringInput::new(String::from("みずほ"))),
                Box::new(String::from("瑞穂")),
                42,
            );
        }
        {
            let _view = EntryView::new(
                &StringInput::new(String::from("みずほ")),
                &String::from("瑞穂"),
                42,
            );
        }
    }

    #[test]
    fn from_view_and_from_entry() {
        let entry1 = Entry::new(
            Box::new(StringInput::new(String::from("みずほ"))),
            Box::new(String::from("瑞穂")),
            42,
        );
        let view = EntryView::from_entry(&entry1);
        let entry2 = Entry::from_view(&view);

        assert_eq!(
            entry1.key().unwrap().as_any().downcast_ref::<StringInput>(),
            view.key().unwrap().as_any().downcast_ref::<StringInput>()
        );
        assert_eq!(
            entry1.key().unwrap().as_any().downcast_ref::<StringInput>(),
            entry2.key().unwrap().as_any().downcast_ref::<StringInput>()
        );
        assert_eq!(
            entry1.value().unwrap().as_any().downcast_ref::<String>(),
            view.value().unwrap().as_any().downcast_ref::<String>()
        );
        assert_eq!(
            entry1.value().unwrap().as_any().downcast_ref::<String>(),
            entry2.value().unwrap().as_any().downcast_ref::<String>()
        );
        assert_eq!(entry1.cost(), view.cost());
        assert_eq!(entry1.cost(), entry2.cost());
    }

    #[test]
    fn clone() {
        {
            let entry1 = Entry::new(
                Box::new(StringInput::new(String::from("みずほ"))),
                Box::new(String::from("瑞穂")),
                42,
            );
            let entry2 = entry1.clone();

            assert_eq!(
                entry1.key().unwrap().as_any().downcast_ref::<StringInput>(),
                entry2.key().unwrap().as_any().downcast_ref::<StringInput>()
            );
            assert_eq!(
                entry1.value().unwrap().as_any().downcast_ref::<String>(),
                entry2.value().unwrap().as_any().downcast_ref::<String>()
            );
            assert_eq!(entry1.cost(), entry2.cost());
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view1 = EntryView::new(&key, &value, 42);
            let view2 = view1.clone();

            assert_eq!(
                view1.key().unwrap().as_any().downcast_ref::<StringInput>(),
                view2.key().unwrap().as_any().downcast_ref::<StringInput>()
            );
            assert_eq!(
                view1.value().unwrap().as_any().downcast_ref::<String>(),
                view2.value().unwrap().as_any().downcast_ref::<String>()
            );
            assert_eq!(view1.cost(), view2.cost());
        }
    }

    #[test]
    fn key() {
        {
            let entry = Entry::new(
                Box::new(StringInput::new(String::from("みずほ"))),
                Box::new(String::from("瑞穂")),
                42,
            );

            assert!(entry.key().is_some());
            assert!(entry.key().unwrap().as_any().is::<StringInput>());
            assert_eq!(
                entry
                    .key()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap()
                    .value(),
                "みずほ"
            );
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(&key, &value, 42);

            assert!(view.key().is_some());
            assert!(view.key().unwrap().as_any().is::<StringInput>());
            assert_eq!(
                view.key()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap()
                    .value(),
                "みずほ"
            );
        }
    }

    #[test]
    fn value() {
        {
            let entry = Entry::new(
                Box::new(StringInput::new(String::from("みずほ"))),
                Box::new(String::from("瑞穂")),
                42,
            );

            assert!(entry.value().is_some());
            assert!(entry.value().unwrap().as_any().is::<String>());
            assert_eq!(
                entry
                    .value()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<String>()
                    .unwrap(),
                "瑞穂"
            );
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(&key, &value, 42);

            assert!(view.value().is_some());
            assert!(view.value().unwrap().as_any().is::<String>());
            assert_eq!(
                view.value()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<String>()
                    .unwrap(),
                "瑞穂"
            );
        }
    }

    #[test]
    fn cost() {
        {
            let entry = Entry::new(
                Box::new(StringInput::new(String::from("みずほ"))),
                Box::new(String::from("瑞穂")),
                42,
            );

            assert_eq!(entry.cost(), 42);
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(&key, &value, 42);

            assert_eq!(view.cost(), 42);
        }
    }
}
