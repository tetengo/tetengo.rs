/*!
 * An entry.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;

use crate::input::Input;

/**
 * A cloneable any.
 */
pub trait CloneableAny: Any {
    /**
     * Clones this object.
     *
     * # Returns
     * A box of a clone of this object.
     */
    fn clone_box(&self) -> Box<dyn CloneableAny>;

    /**
     * Returns this object as 'Any'.
     *
     * # Returns
     * This object as 'Any'.
     */
    fn as_any(&self) -> &dyn Any;
}

impl<T: Clone + 'static> CloneableAny for T {
    fn clone_box(&self) -> Box<dyn CloneableAny> {
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
    value: Box<dyn CloneableAny>,
    cost: i32,
}

impl Debug for Middle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddleEntry")
            .field("key", &"Box<dyn Input>")
            .field("value", &"Box<dyn CloneableAny>")
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
    /// The BOS/EOS (Beginning/End of Sequence) entry.
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
    pub fn new(key: Box<dyn Input>, value: Box<dyn CloneableAny>, cost: i32) -> Self {
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

    /** TODO: doc */
    pub fn key(&self) -> Option<&dyn Input> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.key.as_ref()),
        }
    }
    /*
        const input* entry::p_key() const
        {
            return std::to_address(m_p_key);
        }
    */

    /** TODO: doc */
    pub fn value(&self) -> Option<&dyn CloneableAny> {
        match self {
            Entry::BosEos => None,
            Entry::Middle(entry) => Some(entry.value.as_ref()),
        }
    }
    /*
        const std::any& entry::value() const
        {
            return m_value;
        }
    */

    /** TODO: doc */
    pub fn cost(&self) -> i32 {
        match self {
            Entry::BosEos => 0,
            Entry::Middle(entry) => entry.cost,
        }
    }
    /*
        int entry::cost() const
        {
            return m_cost;
        }
    */
}

/**
 * An middle entry view.
 */
#[derive(Clone)]
pub struct MiddleView<'a> {
    key: &'a dyn Input,
    value: &'a dyn CloneableAny,
    cost: i32,
}

impl Debug for MiddleView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddleEntryView")
            .field("key", &"Option<&dyn Input>")
            .field("value", &"Option<&'a dyn CloneableAny>")
            .field("cost", &self.cost)
            .finish()
    }
}

/**
 * An entry view.
 */
#[derive(Debug, Clone)]
pub enum EntryView<'a> {
    /// The BOS/EOS (Beginning/End of Sequence) entry.
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
    pub const fn new(key: &'a dyn Input, value: &'a dyn CloneableAny, cost: i32) -> Self {
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

    /** TODO: doc */
    pub fn key(&self) -> Option<&'a dyn Input> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.key),
        }
    }
    /*
        /*!
            \brief Returns the pointer to the key.

            \return The pointer to the key.
        */
        [[nodiscard]] constexpr const input* p_key() const
        {
            return m_p_key;
        }
    */

    /** TODO: doc */
    pub fn value(&self) -> Option<&'a dyn CloneableAny> {
        match self {
            EntryView::BosEos => None,
            EntryView::Middle(middle_view) => Some(middle_view.value),
        }
    }
    /*
        /*!
            \brief Returns the value.

            \return The value.
        */
        [[nodiscard]] constexpr const std::any* value() const
        {
            return m_value;
        }
    */

    /** TODO: doc */
    pub fn cost(&self) -> i32 {
        match self {
            EntryView::BosEos => 0,
            EntryView::Middle(middle_view) => middle_view.cost,
        }
    }
    /*
        /*!
            \brief Returns the cost.

            \return The cost.
        */
        [[nodiscard]] constexpr int cost() const
        {
            return m_cost;
        }
    */
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

    /*
    BOOST_AUTO_TEST_CASE(key)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::entry entry_{ std::make_unique<tetengo::lattice::string_input>(key_mizuho),
                                                  surface_mizuho,
                                                  42 };

            BOOST_TEST_REQUIRE(entry_.p_key());
            BOOST_TEST_REQUIRE(entry_.p_key()->is<tetengo::lattice::string_input>());
            BOOST_TEST(entry_.p_key()->as<tetengo::lattice::string_input>().value() == key_mizuho);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(value)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::entry entry_{ std::make_unique<tetengo::lattice::string_input>(key_mizuho),
                                                  surface_mizuho,
                                                  42 };

            BOOST_TEST(std::any_cast<std::string>(entry_.value()) == surface_mizuho);
        }
    }
    */
    /*
        BOOST_AUTO_TEST_CASE(cost)
        {
            BOOST_TEST_PASSPOINT();

            {
                const tetengo::lattice::entry entry_{ std::make_unique<tetengo::lattice::string_input>(key_mizuho),
                                                      surface_mizuho,
                                                      42 };

                BOOST_TEST(entry_.cost() == 42);
            }
        }
    */
}
