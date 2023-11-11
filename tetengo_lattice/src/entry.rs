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
}

impl<T: Clone + 'static> CloneableAny for T {
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone())
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

    /*
        /*!
            \brief Creates an entry.

            \param p_key A unique pointer to a key.
            \param value A value.
            \param cost  A cost.
        */
        entry::entry(std::unique_ptr<input>&& p_key, std::any value, const int cost) :
        m_p_key{ std::move(p_key) },
        m_value{ std::move(value) },
        m_cost{ cost }
        {}
    */
    /*
        entry::entry(const entry_view& view) :
        m_p_key{ view.p_key() ? view.p_key()->clone() : nullptr },
        m_value{ *view.value() },
        m_cost{ view.cost() }
        {}
    */
    /*
        entry::entry(const entry& another) :
        m_p_key{ another.m_p_key ? another.m_p_key->clone() : nullptr },
        m_value{ another.m_value },
        m_cost{ another.m_cost }
        {}
    */
    /*
        entry::entry(entry&& another) :
        m_p_key{ std::move(another.m_p_key) },
        m_value{ std::move(another.m_value) },
        m_cost{ another.m_cost }
        {}
    */

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
    _key: Option<&'a dyn Input>,
    _value: Option<&'a dyn CloneableAny>,
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
        EntryView::Middle(MiddleView {
            _key: Some(key),
            _value: Some(value),
            cost,
        })
    }

    /*
        /*!
            \brief Creates an entry view.

            \param p_key A pointer to a key.
            \param value A value.
            \param cost  A cost.
        */
        constexpr entry_view(const input* p_key, const std::any* value, int cost) :
        m_p_key{ p_key },
        m_value{ std::move(value) },
        m_cost{ cost }
        {}
    */
    /*
        /*!
            \brief Creates an entry view.

            \param entry An entry.
        */
        entry_view(const entry& entry);
    */

    /** TODO: doc */
    pub fn key(&self) -> Option<&'a dyn Input> {
        None
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
        None
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
        0
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
        let _entry1 = Entry::new(
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
        let _entry2 = entry1.clone();
    }

    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::entry      entry1{ std::make_unique<tetengo::lattice::string_input>(key_mizuho),
                                                  surface_mizuho,
                                                  42 };
            const tetengo::lattice::entry_view view{ entry1 };
            const tetengo::lattice::entry      entry2{ view };
            tetengo::lattice::entry            entry3{ entry2 };
            const tetengo::lattice::entry      entry4{ std::move(entry3) };
        }
    }
    */
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
