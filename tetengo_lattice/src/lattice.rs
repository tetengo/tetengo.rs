/*!
 * A lattice.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::node::Node;

struct _GraphStep<'a> {
    input_tail: usize,
    nodes: Vec<Node<'a>>,
    preceding_edge_costs: Vec<Vec<i32>>,
}

impl<'a> _GraphStep<'a> {
    fn _new(input_tail: usize, nodes: Vec<Node<'a>>, preceding_edge_costs: Vec<Vec<i32>>) -> Self {
        Self {
            input_tail,
            nodes,
            preceding_edge_costs,
        }
    }
}

/*
        // functions

        std::size_t input_tail() const
        {
            return m_input_tail;
        }
*/
/*
        const std::vector<node>& nodes() const
        {
            return m_nodes;
        }
*/
/*
        const std::vector<int>& preceding_edge_costs(const std::size_t index) const
        {
            assert(index < std::size(m_p_preceding_edge_costs));
            assert(m_p_preceding_edge_costs[index]);
            return *m_p_preceding_edge_costs[index];
        }
*/
/*

    private:
        // variables

        std::size_t m_input_tail;

        std::vector<node> m_nodes;

        std::vector<std::unique_ptr<std::vector<int>>> m_p_preceding_edge_costs;
    };
*/

/**
 * A lattice.
 */
#[derive(Debug, Clone, Copy)]
pub struct Lattice {}

impl Lattice {
    /*
           /*!
               \brief Creates a lattice.

               \param vocabulary_ A vocabulary.
           */
           explicit lattice(const vocabulary& vocabulary_);
    */
    /*
           /*!
               \brief Destroys the lattice.
           */
           ~lattice();
    */
    /*
           /*!
               \brief Returns the step count.

               \return The step count.
           */
           [[nodiscard]] std::size_t step_count() const;
    */
    /*
           /*!
               \brief Returns the nodes at the specified step.

               \param step A step.

               \return The nodes.

               \throw std::out_of_rage When step is too large.
           */
           [[nodiscard]] const std::vector<node>& nodes_at(std::size_t step) const;
    */
    /*
           /*!
               \brief Pushes back an input.

               \param p_input A unique pointer to an input.
           */
           void push_back(std::unique_ptr<input>&& p_input);
    */
    /*
           /*!
               \brief Settles this lattice.

               You can modify the lattice after settlement.
               Modification of the lattice after settlement invalidate the EOS node.

               \return The EOS node and its preceding edge costs.
           */
           [[nodiscard]] std::pair<node, std::unique_ptr<std::vector<int>>> settle();
    */

    /*


        class lattice::impl : private boost::noncopyable
        {
        public:
            // constructors and destructor

            explicit impl(const vocabulary& vocabulary_) : m_vocabulary{ vocabulary_ }, m_p_input{}, m_graph{}
            {
                m_graph.push_back(bos_step());
            }
    */
    /*
            // functions

            std::size_t step_count() const
            {
                return std::size(m_graph);
            }
    */
    /*
            const std::vector<node>& nodes_at(const std::size_t step) const
            {
                if (step >= std::size(m_graph))
                {
                    throw std::out_of_range{ "step is too large." };
                }

                return m_graph[step].nodes();
            }
    */
    /*
            void push_back(std::unique_ptr<input>&& p_input)
            {
                if (m_p_input)
                {
                    m_p_input->append(std::move(p_input));
                }
                else
                {
                    m_p_input = std::move(p_input);
                }
    */
    /*
                std::vector<node> nodes{};
                auto              p_node_preceding_edge_costs = std::vector<std::unique_ptr<std::vector<int>>>{};
                for (auto i = static_cast<std::size_t>(0); i < std::size(m_graph); ++i)
                {
                    const auto& step = m_graph[i];

                    const auto p_node_key =
                        m_p_input->create_subrange(step.input_tail(), m_p_input->length() - step.input_tail());
                    const auto found = m_vocabulary.find_entries(*p_node_key);

                    std::vector<std::size_t> preceding_edge_cost_indexes{};
                    preceding_edge_cost_indexes.reserve(std::size(found));
                    for (const auto& e: found)
                    {
                        auto p_preceding_edge_costs = preceding_edge_costs(step, e);
                        preceding_edge_cost_indexes.push_back(std::size(p_node_preceding_edge_costs));
                        p_node_preceding_edge_costs.push_back(std::move(p_preceding_edge_costs));
                    }

                    for (auto j = static_cast<std::size_t>(0); j < std::size(found); ++j)
                    {
                        const auto& entry = found[j];
                        const auto& preceding_edge_costs = *p_node_preceding_edge_costs[preceding_edge_cost_indexes[j]];

                        const auto best_preceding_node_index_ = best_preceding_node_index(step, preceding_edge_costs);
                        const auto best_preceding_path_cost = add_cost(
                            step.nodes()[best_preceding_node_index_].path_cost(),
                            preceding_edge_costs[best_preceding_node_index_]);

                        nodes.emplace_back(
                            entry,
                            std::size(nodes),
                            i,
                            &preceding_edge_costs,
                            best_preceding_node_index_,
                            add_cost(best_preceding_path_cost, entry.cost()));
                    }
                }
                if (std::empty(nodes))
                {
                    throw std::invalid_argument{ "No node is found for the input." };
                }

                m_graph.emplace_back(m_p_input->length(), std::move(nodes), std::move(p_node_preceding_edge_costs));
            }
    */
    /*
            std::pair<node, std::unique_ptr<std::vector<int>>> settle()
            {
                auto       p_preceding_edge_costs = preceding_edge_costs(m_graph.back(), entry_view::bos_eos());
                const auto best_preceding_node_index_ = best_preceding_node_index(m_graph.back(), *p_preceding_edge_costs);
                const auto best_preceding_path_cost = add_cost(
                    m_graph.back().nodes()[best_preceding_node_index_].path_cost(),
                    (*p_preceding_edge_costs)[best_preceding_node_index_]);

                node eos_node{ node::eos(
                    std::size(m_graph) - 1,
                    std::to_address(p_preceding_edge_costs),
                    best_preceding_node_index_,
                    best_preceding_path_cost) };
                return std::make_pair(std::move(eos_node), std::move(p_preceding_edge_costs));
            }
    */
    /*

        private:
            // static functions

            static graph_step bos_step()
            {
                std::vector<std::unique_ptr<std::vector<int>>> p_node_preceding_edge_costs{};
                p_node_preceding_edge_costs.push_back(std::make_unique<std::vector<int>>());
                std::vector<node> nodes{ node::bos(std::to_address(p_node_preceding_edge_costs[0])) };
                return graph_step{ 0, std::move(nodes), std::move(p_node_preceding_edge_costs) };
            }
    */
    /*
            static std::size_t best_preceding_node_index(const graph_step& step, const std::vector<int>& edge_costs)
            {
                assert(!std::empty(step.nodes()));
                auto min_index = static_cast<std::size_t>(0);
                for (auto i = static_cast<std::size_t>(1); i < std::size(step.nodes()); ++i)
                {
                    if (add_cost(step.nodes()[i].path_cost(), edge_costs[i]) <
                        add_cost(step.nodes()[min_index].path_cost(), edge_costs[min_index]))
                    {
                        min_index = i;
                    }
                }
                return min_index;
            }
    */
    /*
            static int add_cost(const int one, const int another)
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

            // variables

            const vocabulary& m_vocabulary;

            std::unique_ptr<input> m_p_input;

            std::vector<graph_step> m_graph;

    */
    /*
            // functions

            std::unique_ptr<std::vector<int>>
            preceding_edge_costs(const graph_step& step, const entry_view& next_entry) const
            {
                assert(!std::empty(step.nodes()));
                std::vector<int> costs{};
                costs.reserve(std::size(step.nodes()));
                std::transform(
                    std::begin(step.nodes()),
                    std::end(step.nodes()),
                    std::back_inserter(costs),
                    [this, &next_entry](const auto& node) {
                        return m_vocabulary.find_connection(node, next_entry).cost();
                    });
                return std::make_unique<std::vector<int>>(std::move(costs));
            }
        };
    */
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_lattice() {
    // }

    /*
    namespace
    {
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
        std::unique_ptr<tetengo::lattice::vocabulary> create_cpp_empty_vocabulary()
        {
            return std::make_unique<tetengo::lattice::unordered_map_vocabulary>(
                std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>>{},
                std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>>{},
                cpp_entry_hash,
                cpp_entry_equal_to);
        }
    */
    /*

    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                      p_vocabulary = create_cpp_vocabulary();
            const tetengo::lattice::lattice lattice_{ *p_vocabulary };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(step_count)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            BOOST_TEST(lattice_.step_count() == 1U);

            lattice_.push_back(to_input("[HakataTosu]"));

            BOOST_TEST(lattice_.step_count() == 2U);

            lattice_.push_back(to_input("[TosuOmuta]"));

            BOOST_TEST(lattice_.step_count() == 3U);

            lattice_.push_back(to_input("[OmutaKumamoto]"));
            BOOST_TEST(lattice_.step_count() == 4U);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(nodes_at)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };
            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));

            {
                const auto& nodes = lattice_.nodes_at(0);

                BOOST_TEST_REQUIRE(std::size(nodes) == 1U);
                const std::vector<int> preceding_edge_costs{};
                BOOST_TEST(
                    nodes[0].value().has_value() == tetengo::lattice::node::bos(&preceding_edge_costs).value().has_value());
                for (std::size_t i = 0; i < std::size(nodes); ++i)
                {
                    BOOST_TEST(nodes[i].index_in_step() == i);
                }
            }
            {
                const auto& nodes = lattice_.nodes_at(1);

                BOOST_TEST_REQUIRE(std::size(nodes) == 2U);
                BOOST_TEST(std::any_cast<std::string>(nodes[0].value()) == "kamome");
                BOOST_TEST(std::any_cast<std::string>(nodes[1].value()) == "local415");
                for (std::size_t i = 0; i < std::size(nodes); ++i)
                {
                    BOOST_TEST(nodes[i].index_in_step() == i);
                }
            }
            {
                const auto& nodes = lattice_.nodes_at(2);

                BOOST_TEST_REQUIRE(std::size(nodes) == 3U);
                BOOST_TEST(std::any_cast<std::string>(nodes[0].value()) == "ariake");
                BOOST_TEST(std::any_cast<std::string>(nodes[1].value()) == "rapid811");
                BOOST_TEST(std::any_cast<std::string>(nodes[2].value()) == "local813");
                for (std::size_t i = 0; i < std::size(nodes); ++i)
                {
                    BOOST_TEST(nodes[i].index_in_step() == i);
                }
            }
            {
                const auto& nodes = lattice_.nodes_at(3);

                BOOST_TEST_REQUIRE(std::size(nodes) == 5U);
                BOOST_TEST(std::any_cast<std::string>(nodes[0].value()) == "mizuho");
                BOOST_TEST(std::any_cast<std::string>(nodes[1].value()) == "sakura");
                BOOST_TEST(std::any_cast<std::string>(nodes[2].value()) == "tsubame");
                BOOST_TEST(std::any_cast<std::string>(nodes[3].value()) == "local815");
                BOOST_TEST(std::any_cast<std::string>(nodes[4].value()) == "local817");
                for (std::size_t i = 0; i < std::size(nodes); ++i)
                {
                    BOOST_TEST(nodes[i].index_in_step() == i);
                }
            }
            {
                BOOST_CHECK_THROW([[maybe_unused]] const auto& nodes = lattice_.nodes_at(4), std::out_of_range);
            }
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(push_back)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            lattice_.push_back(to_input("[HakataTosu]"));
            lattice_.push_back(to_input("[TosuOmuta]"));
            lattice_.push_back(to_input("[OmutaKumamoto]"));
        }
        {
            const auto                p_vocabulary = create_cpp_empty_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            BOOST_CHECK_THROW(lattice_.push_back(to_input("[HakataTosu]")), std::invalid_argument);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(settle)
    {
        BOOST_TEST_PASSPOINT();

        {
            const auto                p_vocabulary = create_cpp_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            {
                const auto eos_node_and_preceding_edge_costs = lattice_.settle();

                BOOST_TEST(eos_node_and_preceding_edge_costs.first.preceding_step() == 0U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.best_preceding_node() == 0U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.path_cost() == 8000);

                const std::vector<int> expected_preceding_edge_costs{ 8000 };
                BOOST_CHECK_EQUAL_COLLECTIONS(
                    std::begin(*eos_node_and_preceding_edge_costs.second),
                    std::end(*eos_node_and_preceding_edge_costs.second),
                    std::begin(expected_preceding_edge_costs),
                    std::end(expected_preceding_edge_costs));
            }

            lattice_.push_back(to_input("[HakataTosu]"));
            {
                const auto eos_node_and_preceding_edge_costs = lattice_.settle();

                BOOST_TEST(eos_node_and_preceding_edge_costs.first.preceding_step() == 1U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.best_preceding_node() == 1U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.path_cost() == 7370);

                const std::vector<int> expected_preceding_edge_costs{ 6000, 6000 };
                BOOST_CHECK_EQUAL_COLLECTIONS(
                    std::begin(*eos_node_and_preceding_edge_costs.second),
                    std::end(*eos_node_and_preceding_edge_costs.second),
                    std::begin(expected_preceding_edge_costs),
                    std::end(expected_preceding_edge_costs));
            }

            lattice_.push_back(to_input("[TosuOmuta]"));
            {
                const auto eos_node_and_preceding_edge_costs = lattice_.settle();

                BOOST_TEST(eos_node_and_preceding_edge_costs.first.preceding_step() == 2U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.best_preceding_node() == 1U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.path_cost() == 4010);

                const std::vector<int> expected_preceding_edge_costs{ 2000, 2000, 3000 };
                BOOST_CHECK_EQUAL_COLLECTIONS(
                    std::begin(*eos_node_and_preceding_edge_costs.second),
                    std::end(*eos_node_and_preceding_edge_costs.second),
                    std::begin(expected_preceding_edge_costs),
                    std::end(expected_preceding_edge_costs));
            }

            lattice_.push_back(to_input("[OmutaKumamoto]"));
            {
                const auto eos_node_and_preceding_edge_costs = lattice_.settle();

                BOOST_TEST(eos_node_and_preceding_edge_costs.first.preceding_step() == 3U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.best_preceding_node() == 2U);
                BOOST_TEST(eos_node_and_preceding_edge_costs.first.path_cost() == 3390);

                const std::vector<int> expected_preceding_edge_costs{ 400, 400, 400, 500, 600 };
                BOOST_CHECK_EQUAL_COLLECTIONS(
                    std::begin(*eos_node_and_preceding_edge_costs.second),
                    std::end(*eos_node_and_preceding_edge_costs.second),
                    std::begin(expected_preceding_edge_costs),
                    std::end(expected_preceding_edge_costs));
            }
        }
        {
            const auto                p_vocabulary = create_cpp_empty_vocabulary();
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            const auto eos_node_and_preceding_edge_costs = lattice_.settle();
        }
    }
     */
}
