/*!
 * An entry.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;

use crate::input::Input;

/**
 * A middle entry.
 */
pub struct MiddleEntry {
    _key: Box<dyn Input>,
    value: Box<dyn Any>,
    cost: i32,
}

impl Debug for MiddleEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddleEntry")
            .field("key", &"Box<dyn Input>")
            .field("value", &self.value)
            .field("cost", &self.cost)
            .finish()
    }
}

/**
 * An entry.
 */
#[derive(Debug)]
pub enum Entry {
    /// The BOS/EOS (Beginning/End of Sequence) entry.
    BosEos,

    /// The middle entry.
    Middle(MiddleEntry),
}

impl Entry {
    /*
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
            Entry::Middle(entry) => Some(entry._key.as_ref()),
        }
    }
    /*
        const input* entry::p_key() const
        {
            return std::to_address(m_p_key);
        }
    */

    /** TODO: doc */
    pub fn value(&self) -> Option<&dyn Any> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bos_eos() {
        let bos_eos = Entry::BosEos;

        assert!(bos_eos.key().is_none());
        assert!(bos_eos.value().is_none());
        assert_eq!(bos_eos.cost(), 0);
    }

    /*
    BOOST_AUTO_TEST_CASE(bos_eos)
    {
        BOOST_TEST_PASSPOINT();

        {
            static auto& bos_eos_ = tetengo::lattice::entry::bos_eos();

            BOOST_TEST(!bos_eos_.p_key());
            BOOST_TEST(!bos_eos_.value().has_value());
            BOOST_TEST(bos_eos_.cost() == 0);
        }
    }
    */
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
