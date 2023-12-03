/*!
 * A hash map vocabulary.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;

use crate::connection::Connection;
use crate::entry::{Entry, EntryView};
use crate::node::Node;
use crate::vocabulary::Vocabulary;

type EntryMap = HashMap<String, Vec<Entry>>;

type ConnectionMap<'a> = HashMap<(EntryView<'a>, EntryView<'a>), i32>;

/**
 * A hash map vocabulary.
 */
#[derive(Clone, Debug)]
pub struct HashMapVocabulary<'a> {
    _entry_map: EntryMap,
    _connection_keys: Vec<(Entry, Entry)>,
    _connection_map: ConnectionMap<'a>,
}

impl HashMapVocabulary<'_> {
    /*
        /*!
            \brief Creates an unordered map vocabulary.

            \param entries        Entries.
            \param connections    Connections.
            \param entry_hash     A hash function for an entry.
            \param entry_equal_to An eqaul_to function for an entry.
        */
        unordered_map_vocabulary(
            std::vector<std::pair<std::string, std::vector<entry>>>   entries,
            std::vector<std::pair<std::pair<entry, entry>, int>>      connections,
            std::function<std::size_t(const entry_view&)>             entry_hash,
            std::function<bool(const entry_view&, const entry_view&)> entry_equal_to);
    */
}

impl<'a> Vocabulary for HashMapVocabulary<'a> {
    fn find_entries(&self, _key: &dyn crate::Input) -> Vec<EntryView<'a>> {
        todo!()
    }

    fn find_connection(&self, _from: &Node<'_>, _to: &EntryView<'_>) -> Connection {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    /*
     namespace
     {
         using key_type = tetengo::lattice::string_input;

         constexpr char operator""_c(const unsigned long long int uc)
         {
             return static_cast<char>(uc);
         }

         const std::string key_mizuho{ 0xE3_c, 0x81_c, 0xBF_c, 0xE3_c, 0x81_c, 0x9A_c, 0xE3_c, 0x81_c, 0xBB_c };

         const std::string surface_mizuho{ 0xE7_c, 0x91_c, 0x9E_c, 0xE7_c, 0xA9_c, 0x82_c };

         const std::string key_sakura{ 0xE3_c, 0x81_c, 0x95_c, 0xE3_c, 0x81_c, 0x8F_c, 0xE3_c, 0x82_c, 0x89_c };

         const std::string surface_sakura1{ 0xE6_c, 0xA1_c, 0x9C_c };

         const std::string surface_sakura2{ 0xE3_c, 0x81_c, 0x95_c, 0xE3_c, 0x81_c, 0x8F_c, 0xE3_c, 0x82_c, 0x89_c };

         tetengo::lattice::node make_node(const tetengo::lattice::entry_view& entry)
         {
             static const std::vector<int> preceding_edge_costs{};
             return tetengo::lattice::node{ entry,
                                            0,
                                            std::numeric_limits<std::size_t>::max(),
                                            &preceding_edge_costs,
                                            std::numeric_limits<std::size_t>::max(),
                                            std::numeric_limits<int>::max() };
         }

         std::size_t cpp_entry_hash(const tetengo::lattice::entry_view& entry)
         {
             return entry.p_key() ? entry.p_key()->hash_value() : 0;
         }

         bool cpp_entry_equal_to(const tetengo::lattice::entry_view& one, const tetengo::lattice::entry_view& another)
         {
             return (!one.p_key() && !another.p_key()) ||
                    (one.p_key() && another.p_key() && *one.p_key() == *another.p_key());
         }

    }
     */

    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>                entries{};
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{};
            const tetengo::lattice::unordered_map_vocabulary                                         vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };
        }
        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(find_entries)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>                entries{};
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{};
            const tetengo::lattice::unordered_map_vocabulary                                         vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            {
                const auto found = vocabulary.find_entries(key_type{ key_mizuho });
                BOOST_TEST(std::empty(found));
            }
            {
                const auto found = vocabulary.find_entries(key_type{ key_sakura });
                BOOST_TEST(std::empty(found));
            }
        }
        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<key_type>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<key_type>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<key_type>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{ std::make_unique<key_type>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{ std::make_unique<key_type>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            {
                const auto found = vocabulary.find_entries(key_type{ key_mizuho });
                BOOST_TEST_REQUIRE(std::size(found) == 1U);
                BOOST_TEST_REQUIRE(found[0].p_key());
                BOOST_TEST_REQUIRE(found[0].p_key()->is<key_type>());
                BOOST_TEST(found[0].p_key()->as<key_type>().value() == key_mizuho);
                BOOST_TEST(*std::any_cast<std::string>(found[0].value()) == surface_mizuho);
                BOOST_TEST(found[0].cost() == 42);
            }
            {
                const auto found = vocabulary.find_entries(key_type{ key_sakura });
                BOOST_TEST_REQUIRE(std::size(found) == 2U);
                BOOST_TEST_REQUIRE(found[0].p_key());
                BOOST_TEST_REQUIRE(found[0].p_key()->is<key_type>());
                BOOST_TEST(found[0].p_key()->as<key_type>().value() == key_sakura);
                BOOST_TEST(*std::any_cast<std::string>(found[0].value()) == surface_sakura1);
                BOOST_TEST(found[0].cost() == 24);
                BOOST_TEST_REQUIRE(found[1].p_key());
                BOOST_TEST_REQUIRE(found[1].p_key()->is<key_type>());
                BOOST_TEST(found[1].p_key()->as<key_type>().value() == key_sakura);
                BOOST_TEST(*std::any_cast<std::string>(found[1].value()) == surface_sakura2);
                BOOST_TEST(found[1].cost() == 2424);
            }
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(find_connection)
    {
        BOOST_TEST_PASSPOINT();

        {
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
                { key_mizuho, { { std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 } } },
                { key_sakura,
                  { { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 },
                    { std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura2, 2424 } } }
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                { std::make_pair(
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_mizuho), surface_mizuho, 42 },
                      tetengo::lattice::entry{
                          std::make_unique<tetengo::lattice::string_input>(key_sakura), surface_sakura1, 24 }),
                  4242 }
            };
            const tetengo::lattice::unordered_map_vocabulary vocabulary{
                std::move(entries), std::move(connections), cpp_entry_hash, cpp_entry_equal_to
            };

            const auto entries_mizuho = vocabulary.find_entries(key_type{ key_mizuho });
            BOOST_TEST_REQUIRE(std::size(entries_mizuho) == 1U);
            const auto entries_sakura = vocabulary.find_entries(key_type{ key_sakura });
            BOOST_TEST_REQUIRE(std::size(entries_sakura) == 2U);

            {
                const auto connection = vocabulary.find_connection(make_node(entries_mizuho[0]), entries_sakura[0]);

                BOOST_TEST(connection.cost() == 4242);
            }
            {
                const auto connection = vocabulary.find_connection(make_node(entries_mizuho[0]), entries_mizuho[0]);

                BOOST_TEST(connection.cost() == std::numeric_limits<int>::max());
            }
        }
    }
    */
}
