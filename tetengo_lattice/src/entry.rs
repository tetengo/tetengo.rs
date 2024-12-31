/*!
 * An entry.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
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
    pub fn new(key: Rc<dyn Input>, value: Rc<dyn Any>, cost: i32) -> Self {
        Entry::Middle(Middle { key, value, cost })
    }

    /**
     * Casts this object to a view.
     *
     * # Arguments
     * * `entry` - An entry.
     */
    pub fn as_view(&self) -> EntryView {
        match self {
            Entry::BosEos => EntryView::BosEos,
            Entry::Middle(middle) => {
                EntryView::new(middle.key.clone(), middle.value.clone(), middle.cost)
            }
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

    pub(crate) fn _key_rc(&self) -> Option<Rc<dyn Input>> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.key.clone()),
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

    pub(crate) fn _value_rc(&self) -> Option<Rc<dyn Any>> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.value.clone()),
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

/**
 * An middle entry view.
 */
#[derive(Clone, Debug)]
pub struct MiddleView {
    key: Rc<dyn Input>,
    value: Rc<dyn Any>,
    cost: i32,
}

/**
 * An entry view.
 */
#[derive(Clone, Debug)]
pub enum EntryView {
    /// The BOS/EOS (Beginning/Ending of Sequence) entry.
    BosEos,

    /// The middle entry.
    Middle(MiddleView),
}

impl EntryView {
    /**
     * Creates an entry view.
     *
     * # Arguments
     * * `key`   - A key.
     * * `value` - A value.
     * * `cost`  - A cost.
     */
    pub const fn new(key: Rc<dyn Input>, value: Rc<dyn Any>, cost: i32) -> Self {
        EntryView::Middle(MiddleView { key, value, cost })
    }

    /**
     * Creates an entry.
     *
     * # Arguments
     * * `view` - An entry view.
     */
    pub fn to_entry(&self) -> Entry {
        match self {
            EntryView::BosEos => Entry::BosEos,
            EntryView::Middle(middle_view) => Entry::new(
                middle_view.key.clone(),
                middle_view.value.clone(),
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
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.key.as_ref()),
        }
    }

    pub(crate) fn key_rc(&self) -> Option<Rc<dyn Input>> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(entry) => Some(entry.key.clone()),
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
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.value.as_ref()),
        }
    }

    pub(crate) fn value_rc(&self) -> Option<Rc<dyn Any>> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(entry) => Some(entry.value.clone()),
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
    use crate::string_input::StringInput;

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
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
                42,
            );
        }
        {
            let _view = EntryView::new(
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
                42,
            );
        }
    }

    #[test]
    fn as_view_and_to_entry() {
        let entry1 = Entry::new(
            Rc::new(StringInput::new(String::from("みずほ"))),
            Rc::new(String::from("瑞穂")),
            42,
        );
        let view = entry1.as_view();
        let entry2 = view.to_entry();

        assert_eq!(
            entry1.key().unwrap().downcast_ref::<StringInput>(),
            view.key().unwrap().downcast_ref::<StringInput>()
        );
        assert_eq!(
            entry1.key().unwrap().downcast_ref::<StringInput>(),
            entry2.key().unwrap().downcast_ref::<StringInput>()
        );
        assert_eq!(
            entry1.value().unwrap().downcast_ref::<String>(),
            view.value().unwrap().downcast_ref::<String>()
        );
        assert_eq!(
            entry1.value().unwrap().downcast_ref::<String>(),
            entry2.value().unwrap().downcast_ref::<String>()
        );
        assert_eq!(entry1.cost(), view.cost());
        assert_eq!(entry1.cost(), entry2.cost());
    }

    #[test]
    fn clone() {
        {
            let entry1 = Entry::new(
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
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
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view1 = EntryView::new(Rc::new(key), Rc::new(value), 42);
            let view2 = view1.clone();

            assert_eq!(
                view1.key().unwrap().downcast_ref::<StringInput>(),
                view2.key().unwrap().downcast_ref::<StringInput>()
            );
            assert_eq!(
                view1.value().unwrap().downcast_ref::<String>(),
                view2.value().unwrap().downcast_ref::<String>()
            );
            assert_eq!(view1.cost(), view2.cost());
        }
    }

    #[test]
    fn key() {
        {
            let entry = Entry::new(
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
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
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(Rc::new(key), Rc::new(value), 42);

            assert!(view.key().is_some());
            assert!(view.key().unwrap().is::<StringInput>());
            assert_eq!(
                view.key()
                    .unwrap()
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
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
                42,
            );

            assert!(entry.value().is_some());
            assert!(entry.value().unwrap().is::<String>());
            assert_eq!(
                entry.value().unwrap().downcast_ref::<String>().unwrap(),
                "瑞穂"
            );
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(Rc::new(key), Rc::new(value), 42);

            assert!(view.value().is_some());
            assert!(view.value().unwrap().is::<String>());
            assert_eq!(
                view.value().unwrap().downcast_ref::<String>().unwrap(),
                "瑞穂"
            );
        }
    }

    #[test]
    fn cost() {
        {
            let entry = Entry::new(
                Rc::new(StringInput::new(String::from("みずほ"))),
                Rc::new(String::from("瑞穂")),
                42,
            );

            assert_eq!(entry.cost(), 42);
        }
        {
            let key = StringInput::new(String::from("みずほ"));
            let value = String::from("瑞穂");
            let view = EntryView::new(Rc::new(key), Rc::new(value), 42);

            assert_eq!(view.cost(), 42);
        }
    }
}
