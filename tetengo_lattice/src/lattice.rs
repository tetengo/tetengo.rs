/*!
 * A lattice.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use crate::entry::EntryView;
use crate::input::Input;
use crate::node::Node;
use crate::vocabulary::Vocabulary;

#[derive(Debug)]
struct GraphStep<'a> {
    input_tail: usize,
    nodes: Vec<Node<'a>>,
    _preceding_edge_costs: Vec<Rc<Vec<i32>>>,
}

impl<'a> GraphStep<'a> {
    fn new(
        input_tail: usize,
        nodes: Vec<Node<'a>>,
        preceding_edge_costs: Vec<Rc<Vec<i32>>>,
    ) -> Self {
        Self {
            input_tail,
            nodes,
            _preceding_edge_costs: preceding_edge_costs,
        }
    }

    fn input_tail(&self) -> usize {
        self.input_tail
    }

    fn nodes(&self) -> &[Node<'a>] {
        &self.nodes
    }

    fn _preceding_edge_costs(&self, index: usize) -> &[i32] {
        assert!(index < self._preceding_edge_costs.len());
        &self._preceding_edge_costs[index]
    }
}

/**
 * A lattice.
 */
pub struct Lattice<'a> {
    vocabulary: &'a dyn Vocabulary,
    input: Option<Box<dyn Input>>,
    graph: Vec<GraphStep<'a>>,
}

impl Debug for Lattice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lattice")
            .field("vocabulary", &"&'a dyn Vocabulary")
            .field("input", &"Box<dyn Input>")
            .field("graph", &self.graph)
            .finish()
    }
}

impl<'a> Lattice<'a> {
    /**
     * Creates a lattice.
     *
     * # Arguments
     * * `vocabulary` - A vocabulary.
     */
    pub fn new(vocabulary: &'a dyn Vocabulary) -> Self {
        let mut self_ = Self {
            vocabulary,
            input: None,
            graph: Vec::new(),
        };
        self_.graph.push(Self::bos_step());
        self_
    }

    fn bos_step() -> GraphStep<'a> {
        let node_preceding_edge_costs = vec![Rc::new(Vec::new())];
        let nodes = vec![Node::bos(node_preceding_edge_costs[0].clone())];
        GraphStep::new(0, nodes, node_preceding_edge_costs)
    }

    /**
     * Returns the step count.
     *
     * # Returns
     * The step count.
     */
    pub fn step_count(&self) -> usize {
        self.graph.len()
    }

    /**
     * Pushes back an input.
     *
     * # Arguments
     * * `input` - An input.
     */
    pub fn push_back(&mut self, input: Box<dyn Input>) -> anyhow::Result<()> {
        if let Some(self_input) = &mut self.input {
            self_input.append(input)?;
        } else {
            self.input = Some(input);
        };
        let self_input = match &self.input {
            Some(self_input) => self_input,
            None => unreachable!(),
        };

        let mut nodes = Vec::new();
        let mut node_preceding_edge_costs = Vec::new();
        for i in 0..self.graph.len() {
            let step = &self.graph[i];

            let node_key = match self_input
                .create_subrange(step.input_tail(), self_input.length() - step.input_tail())
            {
                Ok(node_key) => node_key,
                Err(e) => return Err(e),
            };
            let found = self.vocabulary.find_entries(node_key.as_ref());

            let mut preceding_edge_cost_indexes = Vec::new();
            for e in &found {
                let preceding_edge_costs = self.preceding_edge_costs(step, e);
                preceding_edge_cost_indexes.push(node_preceding_edge_costs.len());
                node_preceding_edge_costs.push(preceding_edge_costs);
            }

            for j in 0..found.len() {
                let entry = &found[j];
                let preceding_edge_costs =
                    &node_preceding_edge_costs[preceding_edge_cost_indexes[j]];
                let best_preceding_node_index_ =
                    Self::best_preceding_node_index(step, preceding_edge_costs.as_slice());
                let best_preceding_path_cost = Self::add_cost(
                    step.nodes[best_preceding_node_index_].path_cost(),
                    preceding_edge_costs[best_preceding_node_index_],
                );
                let new_node = match Node::new_with_entry_view(
                    entry,
                    nodes.len(),
                    i,
                    preceding_edge_costs.clone(),
                    best_preceding_node_index_,
                    best_preceding_path_cost,
                ) {
                    Ok(new_node) => new_node,
                    Err(e) => return Err(e),
                };
                nodes.push(new_node);
            }
        }
        if nodes.is_empty() {
            panic!("No node is found for the input.");
        }

        self.graph.push(GraphStep::new(
            self_input.length(),
            nodes,
            node_preceding_edge_costs,
        ));

        Ok(())
    }

    /*
           /*!
               \brief Pushes back an input.

               \param p_input A unique pointer to an input.
           */
           void push_back(std::unique_ptr<input>&& p_input);
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

    fn preceding_edge_costs(
        &self,
        step: &GraphStep<'_>,
        next_entry: &EntryView<'_>,
    ) -> Rc<Vec<i32>> {
        assert!(!step.nodes().is_empty());
        let costs = step
            .nodes()
            .iter()
            .map(|node| self.vocabulary.find_connection(node, next_entry).cost())
            .collect::<Vec<_>>();
        Rc::new(costs)
    }

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

    fn best_preceding_node_index(step: &GraphStep<'_>, edge_costs: &[i32]) -> usize {
        assert!(!step.nodes().is_empty());
        let mut min_index = 0;
        for i in 1..step.nodes().len() {
            if Self::add_cost(step.nodes()[i].path_cost(), edge_costs[i])
                < Self::add_cost(step.nodes()[min_index].path_cost(), edge_costs[min_index])
            {
                min_index = i;
            }
        }
        min_index
    }

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

    fn add_cost(one: i32, another: i32) -> i32 {
        if one == i32::MAX || another == i32::MAX {
            i32::MAX
        } else {
            one + another
        }
    }

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
           /*!
               \brief Settles this lattice.

               You can modify the lattice after settlement.
               Modification of the lattice after settlement invalidate the EOS node.

               \return The EOS node and its preceding edge costs.
           */
           [[nodiscard]] std::pair<node, std::unique_ptr<std::vector<int>>> settle();
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
}

#[cfg(test)]
mod tests {
    use crate::entry::Entry;
    use crate::hash_map_vocabulary::HashMapVocabulary;

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

    #[test]
    fn new() {
        let vocabulary = create_vocabulary();
        let _lattice = Lattice::new(vocabulary.as_ref());
    }

    #[test]
    fn step_count() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        assert_eq!(lattice.step_count(), 1);

        let result1 = lattice.push_back(to_input("[HakataTosu]"));
        assert!(result1.is_ok());
        assert_eq!(lattice.step_count(), 2);

        let result2 = lattice.push_back(to_input("[TosuOmuta]"));
        assert!(result2.is_ok());
        assert_eq!(lattice.step_count(), 3);

        let result3 = lattice.push_back(to_input("[OmutaKumamoto]"));
        assert!(result3.is_ok());
        assert_eq!(lattice.step_count(), 4);
    }

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
