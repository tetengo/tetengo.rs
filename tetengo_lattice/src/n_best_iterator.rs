/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::constraint::Constraint;
use crate::lattice::Lattice;
use crate::node::Node;
use crate::path::Path;

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator<'a> {
    lattice: &'a Lattice<'a>,
    caps: BinaryHeap<Cap<'a>>,
    _eos_hash: u64,
    constraint: Box<Constraint>,
    path: Path<'a>,
    _index: usize,
}

impl<'a> NBestIterator<'a> {
    /**
     * Creates an iterator.
     *
     * # Arguments
     * * `lattice`    - A lattice.
     * * `eos_node`   - An EOS node.
     * * `constraint` - A constraint.
     */
    pub fn new(lattice: &'a Lattice<'a>, eos_node: Node<'a>, constraint: Box<Constraint>) -> Self {
        let mut self_ = Self {
            lattice,
            caps: BinaryHeap::new(),
            _eos_hash: Self::calc_node_hash(&eos_node),
            constraint,
            path: Path::new(),
            _index: 0,
        };

        let tail_path_cost = eos_node.node_cost();
        let whole_path_cost = eos_node.path_cost();
        self_
            .caps
            .push(Cap::_new(vec![eos_node], tail_path_cost, whole_path_cost));

        self_.path = Self::open_cap(self_.lattice, &mut self_.caps, self_.constraint.as_ref());

        self_
    }

    fn calc_node_hash(node: &Node<'_>) -> u64 {
        let mut hasher = DefaultHasher::new();
        match node.key() {
            Some(key) => hasher.write_u64(key.hash_value()),
            None => 0.hash(&mut hasher),
        }
        node.preceding_step().hash(&mut hasher);
        node.preceding_edge_costs().hash(&mut hasher);
        node.best_preceding_node().hash(&mut hasher);
        node.node_cost().hash(&mut hasher);
        node.path_cost().hash(&mut hasher);
        hasher.finish()
    }

    fn open_cap(
        lattice: &Lattice<'a>,
        caps: &mut BinaryHeap<Cap<'a>>,
        constraint: &Constraint,
    ) -> Path<'a> {
        let mut path = Path::new();
        while !caps.is_empty() {
            let Some(opened) = caps.pop() else {
                unreachable!("caps must not be empty.");
            };

            let mut next_path = opened.tail_path().to_vec();
            let mut tail_path_cost = opened.tail_path_cost();
            let mut nonconforming_path = false;
            let Some(mut node) = opened.tail_path().last() else {
                unreachable!("tail_path must not be empty.");
            };
            while !node.is_bos() {
                let Ok(preceding_nodes) = lattice.nodes_at(node.preceding_step()) else {
                    unreachable!("preceding_step must be within the preceding steps in lattice.");
                };
                for (i, preceding_node) in preceding_nodes.iter().enumerate() {
                    if i == node.best_preceding_node() {
                        continue;
                    }
                    let mut cap_tail_path = next_path.clone();
                    cap_tail_path.push(preceding_node.clone());
                    if !constraint.matches_tail(&cap_tail_path) {
                        continue;
                    }
                    let preceding_edge_cost = node.preceding_edge_costs()[i];
                    let cap_tail_path_cost = Self::add_cost(
                        Self::add_cost(tail_path_cost, preceding_edge_cost),
                        preceding_node.node_cost(),
                    );
                    if cap_tail_path_cost == i32::MAX {
                        continue;
                    }
                    let cap_whole_path_cost = Self::add_cost(
                        Self::add_cost(tail_path_cost, preceding_edge_cost),
                        preceding_node.path_cost(),
                    );
                    if cap_whole_path_cost == i32::MAX {
                        continue;
                    }
                    caps.push(Cap::_new(
                        cap_tail_path,
                        cap_tail_path_cost,
                        cap_whole_path_cost,
                    ));
                }

                let best_preceding_edge_cost =
                    node.preceding_edge_costs()[node.best_preceding_node()];
                let best_preceding_node = &preceding_nodes[node.best_preceding_node()];
                next_path.push(best_preceding_node.clone());
                if !constraint.matches_tail(&next_path) {
                    nonconforming_path = true;
                    break;
                }
                tail_path_cost = Self::add_cost(
                    tail_path_cost,
                    Self::add_cost(best_preceding_edge_cost, best_preceding_node.node_cost()),
                );

                node = best_preceding_node;
            }

            if !nonconforming_path {
                assert!(constraint.matches(&next_path));
                let reversed_next_path = next_path.iter().rev().cloned().collect();
                path = Path::new_with_nodes(reversed_next_path, opened.whole_path_cost());
                break;
            }
        }

        path
    }

    fn add_cost(one: i32, another: i32) -> i32 {
        if one == i32::MAX || another == i32::MAX {
            i32::MAX
        } else {
            one + another
        }
    }
}

impl Iterator for NBestIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    /*
        // functions

        /*!
            \brief Dereferences the iterator.

            \return The dereferenced value.
        */
        [[nodiscard]] const path& operator*() const;
    */
    /*
    const path& n_best_iterator::operator*() const
    {
        if (std::empty(m_path))
        {
            throw std::logic_error{ "No more path." };
        }

        return m_path;
    }
    */
    /*
        /*!
            \brief Dereferences the iterator.

            \return The dereferenced value.
        */
        [[nodiscard]] path& operator*();
    */
    /*
    path& n_best_iterator::operator*()
    {
        if (std::empty(m_path))
        {
            throw std::logic_error{ "No more path." };
        }

        return m_path;
    }
    */
    /*
        /*!
            \brief Returns the pointer to the value.

            \return The pointer to the value.
        */
        [[nodiscard]] const path* operator->() const;
    */
    /*
    const path* n_best_iterator::operator->() const
    {
        return &operator*();
    }
    */
    /*
        /*!
            \brief Returns the pointer to the value.

            \return The pointer to the value.
        */
        [[nodiscard]] path* operator->();
    */
    /*
    path* n_best_iterator::operator->()
    {
        return &operator*();
    }
    */
    /*
        /*!
            \brief Returns true when one iterator is equal to another.

            \param one   One iterator.
            \param another Another iterator.

            \retval true  When one is equal to another.
            \retval false Otherwise.
        */
        friend bool operator==(const n_best_iterator& one, const n_best_iterator& another);
    */
    /*
    bool operator==(const n_best_iterator& one, const n_best_iterator& another)
    {
        if (std::empty(one.m_path) && std::empty(another.m_path))
        {
            return true;
        }

        return one.m_p_lattice == another.m_p_lattice && one.m_eos_hash == another.m_eos_hash &&
               one.m_index == another.m_index;
    }
    */
    /*
        /*!
            \brief Increments the iterator.

            \return This iterator.
        */
        n_best_iterator& operator++();
    */
    /*
    n_best_iterator& n_best_iterator::operator++()
    {
        if (std::empty(m_path))
        {
            throw std::logic_error{ "No more path." };
        }

        if (std::empty(m_caps))
        {
            m_path = path{};
        }
        else
        {
            m_path = open_cap(*m_p_lattice, m_caps, *m_p_constraint);
        }
        ++m_index;

        return *this;
    }
    */
    /*
       /*!
           \brief Postincrements the iterator.

           \return The iterator before the incrementation.
       */
       n_best_iterator operator++(int);
    */
    /*
    n_best_iterator n_best_iterator::operator++(int)
    {
        n_best_iterator original{ *this };
        ++(*this);
        return original;
    }
     */
}

#[derive(Debug, Eq)]
struct Cap<'a> {
    tail_path: Vec<Node<'a>>,
    tail_path_cost: i32,
    whole_path_cost: i32,
}

impl<'a> Cap<'a> {
    fn _new(tail_path: Vec<Node<'a>>, tail_path_cost: i32, whole_path_cost: i32) -> Self {
        Cap {
            tail_path,
            tail_path_cost,
            whole_path_cost,
        }
    }

    fn tail_path(&self) -> &[Node<'a>] {
        self.tail_path.as_slice()
    }

    fn tail_path_cost(&self) -> i32 {
        self.tail_path_cost
    }

    fn whole_path_cost(&self) -> i32 {
        self.whole_path_cost
    }
}

impl Ord for Cap<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.whole_path_cost.cmp(&other.whole_path_cost)
    }
}

impl PartialEq for Cap<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.whole_path_cost == other.whole_path_cost
    }
}

impl PartialOrd for Cap<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.whole_path_cost.cmp(&other.whole_path_cost))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::entry::{Entry, EntryView};
    use crate::hash_map_vocabulary::HashMapVocabulary;
    use crate::input::Input;
    use crate::vocabulary::Vocabulary;

    use super::*;

    fn to_input(string: &str) -> Box<dyn Input> {
        Box::new(crate::string_input::StringInput::new(string.to_string()))
    }

    /*
                   +------------------mizuho/sakura/tsubame-------------------+
                   |                path cost: 4270/3220/2990                 |
                   |                                                          |
                   +------------ariake/rapid811------------+                  |
                   |          path cost: 2850/2010         |                  |
                   |                                       |                  |
        BOS--(Hakata)--kamome/local415--(Tosu)--local813--(Omuta)--local817--(Kumamoto)--EOS
                     path cost: 1640/1370   |   pc: 2830           pc: 3160   |     path cost:3390
                                            |                                 |
                                            +------------local815-------------+
                                                      path cost: 3550

        (0) 3390  BOS - tsubame - EOS
            [ sakura(3620),   local817(3760), local815(4050), mizuho(4670)   ]
        (1) 3620  BOS - sakura - EOS
            [ local817(3760), local815(4050), mizuho(4670)                   ]
        (2) 3760  BOS - rapid811 - local817 - EOS
            [ local815(4050), ariake(4600),   mizuho(4670),   local813(4680) ]
        (3) 4050  BOS - local415 - local815 - EOS
            [ kamome(4320),   ariake(4600),   mizuho(4670),   local813(4680) ]
        (4) 4320  BOS - kamome - local815 - EOS
            [ ariake(4600),   mizuho(4670),   local813(4680)                 ]
        (5) 4600  BOS - ariake - local817 - EOS
            [ mizuho(4670),   local813(4680)                                 ]
        (6) 4670  BOS - mizuho - EOS
            [ local813(4680)                                                 ]
        (7) 4680  BOS - local415 - local813 - local817 - EOS
            [ kamome(4950)                                                   ]
        (8) 4950  BOS - kamome - local813 - local817 - EOS
            [                                                                ]
        (9) ----  -
            [                                                                ]
    */
    fn entries() -> Vec<(String, Vec<Entry>)> {
        vec![
            (
                String::from("[HakataTosu][TosuOmuta][OmutaKumamoto]"),
                vec![
                    Entry::new(
                        to_input("Hakata-Tosu-Omuta-Kumamoto"),
                        Box::new("mizuho"),
                        3670,
                    ),
                    Entry::new(
                        to_input("Hakata-Tosu-Omuta-Kumamoto"),
                        Box::new("sakura"),
                        2620,
                    ),
                    Entry::new(
                        to_input("Hakata-Tosu-Omuta-Kumamoto"),
                        Box::new("tsubame"),
                        2390,
                    ),
                ],
            ),
            (
                String::from("[HakataTosu][TosuOmuta]"),
                vec![
                    Entry::new(to_input("Hakata-Tosu-Omuta"), Box::new("ariake"), 2150),
                    Entry::new(to_input("Hakata-Tosu-Omuta"), Box::new("rapid811"), 1310),
                ],
            ),
            (
                String::from("[HakataTosu]"),
                vec![
                    Entry::new(to_input("Hakata-Tosu"), Box::new("kamome"), 840),
                    Entry::new(to_input("Hakata-Tosu"), Box::new("local415"), 570),
                ],
            ),
            (
                String::from("[TosuOmuta]"),
                vec![Entry::new(
                    to_input("Tosu-Omuta"),
                    Box::new("local813"),
                    860,
                )],
            ),
            (
                String::from("[TosuOmuta][OmutaKumamoto]"),
                vec![Entry::new(
                    to_input("Tosu-Omuta-Kumamoto"),
                    Box::new("local815"),
                    1680,
                )],
            ),
            (
                String::from("[OmutaKumamoto]"),
                vec![Entry::new(
                    to_input("Omuta-Kumamoto"),
                    Box::new("local817"),
                    950,
                )],
            ),
        ]
    }

    fn connections() -> Vec<((Entry, Entry), i32)> {
        vec![
            (
                (
                    Entry::BosEos,
                    Entry::new(to_input("Hakata-Tosu-Omuta-Kumamoto"), Box::new(""), 0),
                ),
                600,
            ),
            (
                (
                    Entry::BosEos,
                    Entry::new(to_input("Hakata-Tosu-Omuta"), Box::new(""), 0),
                ),
                700,
            ),
            (
                (
                    Entry::BosEos,
                    Entry::new(to_input("Hakata-Tosu"), Box::new(""), 0),
                ),
                800,
            ),
            ((Entry::BosEos, Entry::BosEos), 8000),
            (
                (
                    Entry::new(to_input("Hakata-Tosu"), Box::new(""), 0),
                    Entry::new(to_input("Tosu-Omuta-Kumamoto"), Box::new(""), 0),
                ),
                500,
            ),
            (
                (
                    Entry::new(to_input("Hakata-Tosu"), Box::new(""), 0),
                    Entry::new(to_input("Tosu-Omuta"), Box::new(""), 0),
                ),
                600,
            ),
            (
                (
                    Entry::new(to_input("Hakata-Tosu"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                6000,
            ),
            (
                (
                    Entry::new(to_input("Hakata-Tosu-Omuta"), Box::new(""), 0),
                    Entry::new(to_input("Omuta-Kumamoto"), Box::new(""), 0),
                ),
                200,
            ),
            (
                (
                    Entry::new(to_input("Hakata-Tosu-Omuta"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                2000,
            ),
            (
                (
                    Entry::new(to_input("Tosu-Omuta"), Box::new(""), 0),
                    Entry::new(to_input("Omuta-Kumamoto"), Box::new(""), 0),
                ),
                300,
            ),
            (
                (
                    Entry::new(to_input("Tosu-Omuta"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                3000,
            ),
            (
                (
                    Entry::new(to_input("Hakata-Tosu-Omuta-Kumamoto"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                400,
            ),
            (
                (
                    Entry::new(to_input("Tosu-Omuta-Kumamoto"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                500,
            ),
            (
                (
                    Entry::new(to_input("Omuta-Kumamoto"), Box::new(""), 0),
                    Entry::BosEos,
                ),
                600,
            ),
        ]
    }

    fn entry_hash(entry: &EntryView<'_>) -> u64 {
        entry.key().map_or(0, |key| key.hash_value())
    }

    fn entry_equal_to(one: &EntryView<'_>, other: &EntryView<'_>) -> bool {
        if one.key().is_none() && other.key().is_none() {
            return true;
        }
        if let Some(one_key) = one.key() {
            if let Some(other_key) = other.key() {
                return one_key.equal_to(other_key);
            }
        }
        false
    }

    fn create_vocabulary() -> Box<dyn Vocabulary> {
        Box::new(HashMapVocabulary::new(
            entries(),
            connections(),
            &entry_hash,
            &entry_equal_to,
        ))
    }

    /*
    int preceding_edge_cost(const tetengo::lattice::path& path_, const std::size_t node_index)
    {
        const auto& nodes = path_.nodes();
        assert(!std::empty(nodes));
        assert(0 < node_index && node_index < std::size(nodes));
        return nodes[node_index].preceding_edge_costs()[nodes[node_index - 1].index_in_step()];
    }
    */
    /*
    int recalc_path_cost(const tetengo::lattice::path& path_)
    {
        const auto& nodes = path_.nodes();
        assert(!std::empty(nodes));
        auto cost = nodes[0].node_cost();
        for (std::size_t i = 1; i < std::size(nodes); ++i)
        {
            cost += preceding_edge_cost(path_, i);
            cost += nodes[i].node_cost();
        }
        return cost;
    }
     */

    #[test]
    fn new() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        let _result = lattice.push_back(to_input("[HakataTosu]"));
        let _result = lattice.push_back(to_input("[TosuOmuta]"));
        let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

        let (eos_node, _) = lattice.settle().unwrap();
        let _iterator = NBestIterator::new(&lattice, eos_node, Box::new(Constraint::new()));
    }

    #[test]
    fn next() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        let _result = lattice.push_back(to_input("[HakataTosu]"));
        let _result = lattice.push_back(to_input("[TosuOmuta]"));
        let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

        let (eos_node, _) = lattice.settle().unwrap();
        let _iterator = NBestIterator::new(&lattice, eos_node, Box::new(Constraint::new()));
    }
    /*
    BOOST_AUTO_TEST_CASE(operator_dereference)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto                                    eos_node_and_preceding_edge_costs = lattice_.settle();
            const tetengo::lattice::n_best_iterator iterator{ lattice_,
                                                              std::move(eos_node_and_preceding_edge_costs.first),
                                                              std::make_unique<tetengo::lattice::constraint>() };

            const auto& path = *iterator;
            BOOST_TEST_REQUIRE(std::size(path.nodes()) == 3U);
            BOOST_TEST(!path.nodes()[0].value().has_value());
            BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "tsubame");
            BOOST_TEST(!path.nodes()[2].value().has_value());
        }
        {
            const tetengo::lattice::n_best_iterator iterator{};

            BOOST_CHECK_THROW([[maybe_unused]] const auto& dereferenced = *iterator, std::logic_error);
        }
    }
        */
    /*
    BOOST_AUTO_TEST_CASE(operator_equal)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto                                    eos_node_and_preceding_edge_costs1 = lattice_.settle();
            const tetengo::lattice::n_best_iterator iterator1{ lattice_,
                                                               std::move(eos_node_and_preceding_edge_costs1.first),
                                                               std::make_unique<tetengo::lattice::constraint>() };
            auto                                    eos_node_and_preceding_edge_costs2 = lattice_.settle();
            const tetengo::lattice::n_best_iterator iterator2{ lattice_,
                                                               std::move(eos_node_and_preceding_edge_costs2.first),
                                                               std::make_unique<tetengo::lattice::constraint>() };
            auto                                    eos_node_and_preceding_edge_costs3 = lattice_.settle();
            tetengo::lattice::n_best_iterator       iterator3{ lattice_,
                                                         std::move(eos_node_and_preceding_edge_costs3.first),
                                                         std::make_unique<tetengo::lattice::constraint>() };
            ++iterator3;
            const tetengo::lattice::n_best_iterator iterator_last{};

            BOOST_CHECK(iterator1 == iterator1);
            BOOST_CHECK(iterator1 == iterator2);
            BOOST_CHECK(iterator1 != iterator3);
            BOOST_CHECK(iterator1 != iterator_last);

            ++iterator3;
            ++iterator3;
            ++iterator3;
            ++iterator3;
            ++iterator3;
            ++iterator3;
            ++iterator3;
            ++iterator3;

            BOOST_CHECK(iterator3 == iterator_last);
        }
        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));

            auto                                    eos_node_and_preceding_edge_costs1 = lattice_.settle();
            const tetengo::lattice::n_best_iterator iterator1{ lattice_,
                                                               std::move(eos_node_and_preceding_edge_costs1.first),
                                                               std::make_unique<tetengo::lattice::constraint>() };

            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto                                    eos_node_and_preceding_edge_costs2 = lattice_.settle();
            const tetengo::lattice::n_best_iterator iterator2{ lattice_,
                                                               std::move(eos_node_and_preceding_edge_costs2.first),
                                                               std::make_unique<tetengo::lattice::constraint>() };

            BOOST_CHECK(iterator1 != iterator2);
        }
    }
        */
    /*
    BOOST_AUTO_TEST_CASE(operator_increment)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto                              eos_node_and_preceding_edge_costs = lattice_.settle();
            tetengo::lattice::n_best_iterator iterator{ lattice_,
                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                        std::make_unique<tetengo::lattice::constraint>() };
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 3U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "tsubame");
                BOOST_TEST(preceding_edge_cost(path, 1) == 600);
                BOOST_TEST(!path.nodes()[2].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 2) == 400);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            ++iterator;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 3U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "sakura");
                BOOST_TEST(preceding_edge_cost(path, 1) == 600);
                BOOST_TEST(!path.nodes()[2].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 2) == 400);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "rapid811");
                BOOST_TEST(preceding_edge_cost(path, 1) == 700);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local817");
                BOOST_TEST(preceding_edge_cost(path, 2) == 200);
                BOOST_TEST(!path.nodes()[3].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 3) == 600);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            ++iterator;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "local415");
                BOOST_TEST(preceding_edge_cost(path, 1) == 800);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local815");
                BOOST_TEST(preceding_edge_cost(path, 2) == 500);
                BOOST_TEST(!path.nodes()[3].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 3) == 500);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "kamome");
                BOOST_TEST(preceding_edge_cost(path, 1) == 800);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local815");
                BOOST_TEST(preceding_edge_cost(path, 2) == 500);
                BOOST_TEST(!path.nodes()[3].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 3) == 500);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            ++iterator;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "ariake");
                BOOST_TEST(preceding_edge_cost(path, 1) == 700);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local817");
                BOOST_TEST(preceding_edge_cost(path, 2) == 200);
                BOOST_TEST(!path.nodes()[3].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 3) == 600);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 3U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "mizuho");
                BOOST_TEST(preceding_edge_cost(path, 1) == 600);
                BOOST_TEST(!path.nodes()[2].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 2) == 400);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            ++iterator;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 5U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "local415");
                BOOST_TEST(preceding_edge_cost(path, 1) == 800);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local813");
                BOOST_TEST(preceding_edge_cost(path, 2) == 600);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[3].value()) == "local817");
                BOOST_TEST(preceding_edge_cost(path, 3) == 300);
                BOOST_TEST(!path.nodes()[4].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 4) == 600);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 5U);
                BOOST_TEST(!path.nodes()[0].value().has_value());
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[1].value()) == "kamome");
                BOOST_TEST(preceding_edge_cost(path, 1) == 800);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[2].value()) == "local813");
                BOOST_TEST(preceding_edge_cost(path, 2) == 600);
                BOOST_TEST(std::any_cast<std::string>(path.nodes()[3].value()) == "local817");
                BOOST_TEST(preceding_edge_cost(path, 3) == 300);
                BOOST_TEST(!path.nodes()[4].value().has_value());
                BOOST_TEST(preceding_edge_cost(path, 4) == 600);
                BOOST_TEST(recalc_path_cost(path) == path.cost());
            }

            ++iterator;
            BOOST_CHECK_THROW([[maybe_unused]] const auto& dereferenced = *iterator, std::logic_error);
        }
        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto                              eos_node_and_preceding_edge_costs = lattice_.settle();
            tetengo::lattice::n_best_iterator iterator{ lattice_,
                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                        std::make_unique<tetengo::lattice::constraint>() };

            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 3U);

                std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
                pattern.reserve(3);
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[0]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[1]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[2]));
                auto p_constraint = std::make_unique<tetengo::lattice::constraint>(std::move(pattern));

                tetengo::lattice::n_best_iterator constrained_iterator{ lattice_,
                                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                                        std::move(p_constraint) };

                BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                const auto& constrained_path = *iterator;
                BOOST_TEST(constrained_path.nodes() == path.nodes());

                ++constrained_iterator;
                BOOST_CHECK(constrained_iterator == tetengo::lattice::n_best_iterator{});
            }

            ++iterator;
            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);

                std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
                pattern.reserve(4);
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[0]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[1]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[2]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[3]));
                auto p_constraint = std::make_unique<tetengo::lattice::constraint>(std::move(pattern));

                tetengo::lattice::n_best_iterator constrained_iterator{ lattice_,
                                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                                        std::move(p_constraint) };

                BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                const auto& constrained_path = *constrained_iterator;
                BOOST_TEST(constrained_path.nodes() == path.nodes());

                constrained_iterator++;
                BOOST_CHECK(constrained_iterator == tetengo::lattice::n_best_iterator{});
            }

            ++iterator;
            iterator++;
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);

                std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
                pattern.reserve(4);
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[0]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[1]));
                pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(1));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[3]));
                auto p_constraint = std::make_unique<tetengo::lattice::constraint>(std::move(pattern));

                tetengo::lattice::n_best_iterator constrained_iterator{ lattice_,
                                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                                        std::move(p_constraint) };

                {
                    BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                    const auto& constrained_path = *constrained_iterator;
                    BOOST_TEST(constrained_path.nodes() == path.nodes());
                }
                ++constrained_iterator;
                {
                    BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                    const auto& constrained_path = *constrained_iterator;
                    BOOST_TEST_REQUIRE(std::size(constrained_path.nodes()) == 5U);
                    BOOST_CHECK(constrained_path.nodes()[0] == path.nodes()[0]);
                    BOOST_CHECK(constrained_path.nodes()[1] == path.nodes()[1]);
                    BOOST_TEST_REQUIRE(constrained_path.nodes()[2].p_key());
                    BOOST_TEST_REQUIRE(constrained_path.nodes()[2].p_key()->is<tetengo::lattice::string_input>());
                    BOOST_TEST(
                        constrained_path.nodes()[2].p_key()->as<tetengo::lattice::string_input>().value() == "Tosu-Omuta");
                    BOOST_TEST_REQUIRE(constrained_path.nodes()[3].p_key());
                    BOOST_TEST_REQUIRE(constrained_path.nodes()[3].p_key()->is<tetengo::lattice::string_input>());
                    BOOST_TEST(
                        constrained_path.nodes()[3].p_key()->as<tetengo::lattice::string_input>().value() ==
                        "Omuta-Kumamoto");
                    BOOST_CHECK(constrained_path.nodes()[4] == path.nodes()[3]);
                }
                constrained_iterator++;
                {
                    BOOST_CHECK(constrained_iterator == tetengo::lattice::n_best_iterator{});
                }
            }
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);

                std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
                pattern.reserve(4);
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[0]));
                pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(0));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[2]));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[3]));
                auto p_constraint = std::make_unique<tetengo::lattice::constraint>(std::move(pattern));

                tetengo::lattice::n_best_iterator constrained_iterator{ lattice_,
                                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                                        std::move(p_constraint) };

                {
                    BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                    const auto& constrained_path = *constrained_iterator;
                    BOOST_TEST_REQUIRE(std::size(constrained_path.nodes()) == 4U);
                    BOOST_CHECK(constrained_path.nodes()[0] == path.nodes()[0]);
                    BOOST_CHECK(std::any_cast<std::string>(constrained_path.nodes()[1].value()) == "local415");
                    BOOST_CHECK(constrained_path.nodes()[2] == path.nodes()[2]);
                    BOOST_CHECK(constrained_path.nodes()[3] == path.nodes()[3]);
                }
                ++constrained_iterator;
                {
                    BOOST_REQUIRE(constrained_iterator != tetengo::lattice::n_best_iterator{});
                    const auto& constrained_path = *constrained_iterator;
                    BOOST_TEST(constrained_path.nodes() == path.nodes());
                }
                constrained_iterator++;
                {
                    BOOST_CHECK(constrained_iterator == tetengo::lattice::n_best_iterator{});
                }
            }
            {
                const auto& path = *iterator;
                BOOST_TEST_REQUIRE(std::size(path.nodes()) == 4U);

                std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
                pattern.reserve(4);
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[0]));
                pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(0));
                pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path.nodes()[3]));
                auto p_constraint = std::make_unique<tetengo::lattice::constraint>(std::move(pattern));

                tetengo::lattice::n_best_iterator constrained_iterator{ lattice_,
                                                                        std::move(eos_node_and_preceding_edge_costs.first),
                                                                        std::move(p_constraint) };

                BOOST_TEST(std::distance(constrained_iterator, tetengo::lattice::n_best_iterator{}) == 9);
            }
        }
    }
         */

    mod cap {
        use super::*;

        #[test]
        fn new() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let _cap = Cap::_new(nodes, 24, 42);
        }

        #[test]
        fn ord() {
            let preceding_edge_costs1 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node1 = Node::eos(1, preceding_edge_costs1, 5, 42);
            let nodes1 = vec![node1];
            let cap1 = Cap::_new(nodes1, 24, 42);

            let preceding_edge_costs2 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node2 = Node::eos(1, preceding_edge_costs2, 5, 42);
            let nodes2 = vec![node2];
            let cap2 = Cap::_new(nodes2, 24, 42);

            let preceding_edge_costs3 = Rc::new(vec![2, 7, 1, 8, 2, 8]);
            let node3 = Node::eos(2, preceding_edge_costs3, 3, 31);
            let nodes3 = vec![node3];
            let cap3 = Cap::_new(nodes3, 12, 4242);

            assert!(cap1 == cap2);
            assert!(cap1 < cap3);
        }

        #[test]
        fn tail_path() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs.clone(), 5, 42);
            let nodes = vec![node];
            let cap = Cap::_new(nodes, 24, 42);

            assert_eq!(cap.tail_path().len(), 1);
            assert_eq!(
                cap.tail_path()[0].preceding_edge_costs(),
                preceding_edge_costs.as_slice()
            );
        }

        #[test]
        fn tail_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = Cap::_new(nodes, 24, 42);

            assert_eq!(cap.tail_path_cost(), 24);
        }

        #[test]
        fn whole_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = Cap::_new(nodes, 24, 42);

            assert_eq!(cap.whole_path_cost(), 42);
        }
    }
}
