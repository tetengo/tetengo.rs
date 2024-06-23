/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use crate::constraint::Constraint;
use crate::lattice::Lattice;
use crate::node::Node;
use crate::path::Path;

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator<'a> {
    _lattice: &'a Lattice<'a>,
    _caps: BinaryHeap<_Cap<'a>>,
    _eos_hash: u64,
    _constraint: Rc<Constraint>,
    _path: Path<'a>,
    _index: usize,
}

impl NBestIterator<'_> {
    /*
        // constructors and destructor

        /*!
            \brief Creates an iterator.

            It points to the last of the paths.
        */
        n_best_iterator();
    */
    /*
    n_best_iterator::n_best_iterator() :
    m_p_lattice{},
    m_caps{},
    m_eos_hash{ 0 },
    m_p_constraint{ std::make_shared<constraint>() },
    m_path{},
    m_index{ 0 }
    {}
    */
    /*
        /*!
            \brief Creates an iterator.

            \param lattice_     A lattice.
            \param eos_node     An EOS node.
            \param p_constraint A unique pointer to a constraint.

            \throw std::invalid_argument When p_constraint is nullptr.
        */
        n_best_iterator(const lattice& lattice_, node eos_node, std::unique_ptr<constraint>&& p_constraint);
    */
    /*
    n_best_iterator::n_best_iterator(
        const lattice&                lattice_,
        node                          eos_node,
        std::unique_ptr<constraint>&& p_constraint) :
    m_p_lattice{ &lattice_ },
    m_caps{},
    m_eos_hash{ calc_node_hash(eos_node) },
    m_p_constraint{ std::move(p_constraint) },
    m_path{},
    m_index{ 0 }
    {
        if (!m_p_constraint)
        {
            throw std::invalid_argument{ "p_constraint is nullptr." };
        }

        const int tail_path_cost = eos_node.node_cost();
        const int whole_path_cost = eos_node.path_cost();
        m_caps.emplace(std::vector<node>{ std::move(eos_node) }, tail_path_cost, whole_path_cost);

        m_path = open_cap(*m_p_lattice, m_caps, *m_p_constraint);
    }
    */
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
    /*
    namespace
    {
        std::size_t calc_node_hash(const node& node_)
        {
            auto seed = static_cast<std::size_t>(0);
            boost::hash_combine(seed, node_.p_key() ? node_.p_key()->hash_value() : 0);
            boost::hash_combine(seed, boost::hash_value(node_.preceding_step()));
            boost::hash_combine(seed, boost::hash_value(node_.preceding_edge_costs()));
            boost::hash_combine(seed, boost::hash_value(node_.best_preceding_node()));
            boost::hash_combine(seed, boost::hash_value(node_.node_cost()));
            boost::hash_combine(seed, boost::hash_value(node_.path_cost()));
            return seed;
        }
    */
    /*
        int add_cost(const int one, const int another)
        {
            if (one == std::numeric_limits<int>::max() || another == std::numeric_limits<int>::max())
            {
                return std::numeric_limits<int>::max();
            }
            else
            {
                return one + another;
            }
        }
    */
    /*
        path open_cap(
            const lattice&                                                 lattice_,
            std::priority_queue<cap, std::vector<cap>, std::greater<cap>>& caps,
            const constraint&                                              constraint_)
        {
            path path_{};
            while (!std::empty(caps))
            {
                const auto opened = caps.top();
                caps.pop();

                auto next_path = opened.tail_path();
                auto tail_path_cost = opened.tail_path_cost();
                bool nonconforming_path = false;
                for (const auto* p_node = &opened.tail_path().back(); !p_node->is_bos();)
                {
                    const auto& preceding_nodes = lattice_.nodes_at(p_node->preceding_step());
                    for (auto i = static_cast<std::size_t>(0); i < std::size(preceding_nodes); ++i)
                    {
                        if (i == p_node->best_preceding_node())
                        {
                            continue;
                        }
                        const auto&       preceding_node = preceding_nodes[i];
                        std::vector<node> cap_tail_path{ next_path };
                        cap_tail_path.push_back(preceding_node);
                        if (!constraint_.matches_tail(cap_tail_path))
                        {
                            continue;
                        }
                        const auto preceding_edge_cost = p_node->preceding_edge_costs()[i];
                        const auto cap_tail_path_cost =
                            add_cost(add_cost(tail_path_cost, preceding_edge_cost), preceding_node.node_cost());
                        if (cap_tail_path_cost == std::numeric_limits<int>::max())
                        {
                            continue;
                        }
                        const auto cap_whole_path_cost =
                            add_cost(add_cost(tail_path_cost, preceding_edge_cost), preceding_node.path_cost());
                        if (cap_whole_path_cost == std::numeric_limits<int>::max())
                        {
                            continue;
                        }
                        caps.emplace(std::move(cap_tail_path), cap_tail_path_cost, cap_whole_path_cost);
                    }

                    const auto best_preceding_edge_cost = p_node->preceding_edge_costs()[p_node->best_preceding_node()];
                    const auto& best_preceding_node = preceding_nodes[p_node->best_preceding_node()];
                    next_path.push_back(best_preceding_node);
                    if (!constraint_.matches_tail(next_path))
                    {
                        nonconforming_path = true;
                        break;
                    }
                    tail_path_cost =
                        add_cost(tail_path_cost, add_cost(best_preceding_edge_cost, best_preceding_node.node_cost()));

                    p_node = &best_preceding_node;
                }

                if (!nonconforming_path)
                {
                    assert(constraint_.matches(next_path));
                    path_ = path{ std::vector<node>{ std::rbegin(next_path), std::rend(next_path) },
                                  opened.whole_path_cost() };
                    break;
                }
            }

            return path_;
        }


    }
    */
}

impl Iterator for NBestIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug, Eq)]
struct _Cap<'a> {
    _tail_path: Vec<Node<'a>>,
    _tail_path_cost: i32,
    whole_path_cost: i32,
}

impl<'a> _Cap<'a> {
    fn _new(tail_path: Vec<Node<'a>>, tail_path_cost: i32, whole_path_cost: i32) -> Self {
        _Cap {
            _tail_path: tail_path,
            _tail_path_cost: tail_path_cost,
            whole_path_cost,
        }
    }

    fn _tail_path(&self) -> &[Node<'a>] {
        self._tail_path.as_slice()
    }

    fn _tail_path_cost(&self) -> i32 {
        self._tail_path_cost
    }

    fn _whole_path_cost(&self) -> i32 {
        self.whole_path_cost
    }
}

impl Ord for _Cap<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.whole_path_cost.cmp(&other.whole_path_cost)
    }
}

impl PartialEq for _Cap<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.whole_path_cost == other.whole_path_cost
    }
}

impl PartialOrd for _Cap<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.whole_path_cost.cmp(&other.whole_path_cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    std::unique_ptr<tetengo::lattice::input> to_input(const char* const string)
    {
        return std::make_unique<tetengo::lattice::string_input>(string);
    }
    */
    /*
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
    const std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entries{
        { "[HakataTosu][TosuOmuta][OmutaKumamoto]",
          {
              { to_input("Hakata-Tosu-Omuta-Kumamoto"), std::string{ "mizuho" }, 3670 },
              { to_input("Hakata-Tosu-Omuta-Kumamoto"), std::string{ "sakura" }, 2620 },
              { to_input("Hakata-Tosu-Omuta-Kumamoto"), std::string{ "tsubame" }, 2390 },
          } },
        { "[HakataTosu][TosuOmuta]",
          {
              { to_input("Hakata-Tosu-Omuta"), std::string{ "ariake" }, 2150 },
              { to_input("Hakata-Tosu-Omuta"), std::string{ "rapid811" }, 1310 },
          } },
        { "[HakataTosu]",
          {
              { to_input("Hakata-Tosu"), std::string{ "kamome" }, 840 },
              { to_input("Hakata-Tosu"), std::string{ "local415" }, 570 },
          } },
        { "[TosuOmuta]",
          {
              { to_input("Tosu-Omuta"), std::string{ "local813" }, 860 },
          } },
        { "[TosuOmuta][OmutaKumamoto]",
          {
              { to_input("Tosu-Omuta-Kumamoto"), std::string{ "local815" }, 1680 },
          } },
        { "[OmutaKumamoto]",
          {
              { to_input("Omuta-Kumamoto"), std::string{ "local817" }, 950 },
          } },
    };
    */
    /*
    const std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
        { { tetengo::lattice::entry::bos_eos(), { to_input("Hakata-Tosu-Omuta-Kumamoto"), {}, 0 } }, 600 },
        { { tetengo::lattice::entry::bos_eos(), { to_input("Hakata-Tosu-Omuta"), {}, 0 } }, 700 },
        { { tetengo::lattice::entry::bos_eos(), { to_input("Hakata-Tosu"), {}, 0 } }, 800 },
        { { tetengo::lattice::entry::bos_eos(), tetengo::lattice::entry::bos_eos() }, 8000 },
        { { { to_input("Hakata-Tosu"), {}, 0 }, { to_input("Tosu-Omuta-Kumamoto"), {}, 0 } }, 500 },
        { { { to_input("Hakata-Tosu"), {}, 0 }, { to_input("Tosu-Omuta"), {}, 0 } }, 600 },
        { { { to_input("Hakata-Tosu"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 6000 },
        { { { to_input("Hakata-Tosu-Omuta"), {}, 0 }, { to_input("Omuta-Kumamoto"), {}, 0 } }, 200 },
        { { { to_input("Hakata-Tosu-Omuta"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 2000 },
        { { { to_input("Tosu-Omuta"), {}, 0 }, { to_input("Omuta-Kumamoto"), {}, 0 } }, 300 },
        { { { to_input("Tosu-Omuta"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 3000 },
        { { { to_input("Hakata-Tosu-Omuta-Kumamoto"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 400 },
        { { { to_input("Tosu-Omuta-Kumamoto"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 500 },
        { { { to_input("Omuta-Kumamoto"), {}, 0 }, tetengo::lattice::entry::bos_eos() }, 600 },
    };
    */
    /*
    std::size_t cpp_entry_hash(const tetengo::lattice::entry_view& entry)
    {
        return entry.p_key() ? entry.p_key()->hash_value() : 0;
    }
    */
    /*
    bool cpp_entry_equal_to(const tetengo::lattice::entry_view& one, const tetengo::lattice::entry_view& another)
    {
        return (!one.p_key() && !another.p_key()) ||
               (one.p_key() && another.p_key() && *one.p_key() == *another.p_key());
    }
    */
    /*
    std::unique_ptr<tetengo::lattice::vocabulary> create_cpp_vocabulary()
    {
        return std::make_unique<tetengo::lattice::unordered_map_vocabulary>(
            entries, connections, cpp_entry_hash, cpp_entry_equal_to);
    }
    */
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
    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::n_best_iterator iterator{};
        }
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
        }
        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            auto eos_node_and_preceding_edge_costs = lattice_.settle();
            BOOST_CHECK_THROW(
                const tetengo::lattice::n_best_iterator iterator(
                    lattice_,
                    std::move(eos_node_and_preceding_edge_costs.first),
                    std::unique_ptr<tetengo::lattice::constraint>()),
                std::invalid_argument);
        }
    }
        */
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
            let _cap = _Cap::_new(nodes, 24, 42);
        }

        #[test]
        fn ord() {
            let preceding_edge_costs1 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node1 = Node::eos(1, preceding_edge_costs1, 5, 42);
            let nodes1 = vec![node1];
            let cap1 = _Cap::_new(nodes1, 24, 42);

            let preceding_edge_costs2 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node2 = Node::eos(1, preceding_edge_costs2, 5, 42);
            let nodes2 = vec![node2];
            let cap2 = _Cap::_new(nodes2, 24, 42);

            let preceding_edge_costs3 = Rc::new(vec![2, 7, 1, 8, 2, 8]);
            let node3 = Node::eos(2, preceding_edge_costs3, 3, 31);
            let nodes3 = vec![node3];
            let cap3 = _Cap::_new(nodes3, 12, 4242);

            assert!(cap1 == cap2);
            assert!(cap1 < cap3);
        }

        #[test]
        fn tail_path() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs.clone(), 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._tail_path().len(), 1);
            assert_eq!(
                cap._tail_path()[0].preceding_edge_costs(),
                preceding_edge_costs.as_slice()
            );
        }

        #[test]
        fn tail_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._tail_path_cost(), 24);
        }

        #[test]
        fn whole_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = _Cap::_new(nodes, 24, 42);

            assert_eq!(cap._whole_path_cost(), 42);
        }
    }
}
