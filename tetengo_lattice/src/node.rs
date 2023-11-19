/*!
 * A node.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::entry::AnyValue;
use crate::input::Input;

/**
 * A BOS (Beginning of Sequence) node.
 */
#[derive(Clone, Copy, Debug)]
pub struct Bos<'a> {
    _preceding_edge_costs: &'a Vec<i32>,
}

/**
 * A EOS (Ending of Sequence) node.
 */
#[derive(Clone, Copy, Debug)]
pub struct Eos<'a> {
    _preceding_step: usize,
    _preceding_edge_costs: &'a Vec<i32>,
    _best_preceding_node: usize,
    _path_cost: i32,
}
/**
 * A middle node.
 */
#[derive(Clone, Copy)]
pub struct Middle<'a> {
    _key: &'a dyn Input,
    _value: &'a dyn AnyValue,
    index_in_step: usize,
    preceding_step: usize,
    preceding_edge_costs: &'a Vec<i32>,
    best_preceding_node: usize,
    node_cost: i32,
    path_cost: i32,
}

impl Debug for Middle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Middle")
            .field("key", &"&'a dyn Input")
            .field("value", &"&'a dyn AnyValue")
            .field("index_in_step", &self.index_in_step)
            .field("preceding_step", &self.preceding_step)
            .field("preceding_edge_costs", &self.preceding_edge_costs)
            .field("best_preceding_node", &self.best_preceding_node)
            .field("node_cost", &self.node_cost)
            .field("path_cost", &self.path_cost)
            .finish()
    }
}

/**
 * A node.
 */
#[derive(Clone, Debug)]
pub enum Node<'a> {
    /// The BOS (Beginning of Sequence) node.
    Bos(Bos<'a>),

    /// The EOS (Ending of Sequence) node.
    Eos(Eos<'a>),

    /// The middle node.
    Middle(Middle<'a>),
}

impl<'a> Node<'a> {
    /**
     * Creates a BOS (Beginning of Sequence).
     *
     * # Arguments
     * * preceding_edge_costs - Preceding edge costs.
     */
    pub const fn bos(preceding_edge_costs: &'a Vec<i32>) -> Self {
        Node::Bos(Bos {
            _preceding_edge_costs: preceding_edge_costs,
        })
    }

    /**
     * Creates an EOS (Ending of Sequence).
     *
     * # Arguments
     * * preceding_step       - An index of a preceding step.
     * * preceding_edge_costs - Preceding edge costs.
     * * best_preceding_node  - An index of a best preceding node.
     * * path_cost            - A path cost.
     */
    pub const fn eos(
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Self {
        Node::Eos(Eos {
            _preceding_step: preceding_step,
            _preceding_edge_costs: preceding_edge_costs,
            _best_preceding_node: best_preceding_node,
            _path_cost: path_cost,
        })
    }

    /**
     * Creates a node.
     *
     * # Arguments
     * * key                  - A key.
     * * value                - A value.
     * * index_in_step        - An index in the step.
     * * preceding_step       - An index of a preceding step.
     * * preceding_edge_costs - Preceding edge costs.
     * * best_preceding_node  - An index of a best preceding node.
     * * node_cost            - A node cost.
     * * path_cost            - A path cost.
     */
    pub const fn new(
        key: &'a dyn Input,
        value: &'a dyn AnyValue,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        node_cost: i32,
        path_cost: i32,
    ) -> Self {
        Node::Middle(Middle {
            _key: key,
            _value: value,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            node_cost,
            path_cost,
        })
    }

    /*
        /*!
            \brief Creates a node from a vocabulary entry.

            \param entry                  An entry.
            \param index_in_step          An index in the step.
            \param preceding_step         An index of a preceding step.
            \param p_preceding_edge_costs A pointer to preceding edge costs.
            \param best_preceding_node    An index of a best preceding node.
            \param path_cost              A path cost.

            \throw std::invalid_argument When p_preceding_edge_costs is nullptr.
        */
        constexpr node(
            const entry_view&       entry,
            std::size_t             index_in_step,
            std::size_t             preceding_step,
            const std::vector<int>* p_preceding_edge_costs,
            std::size_t             best_preceding_node,
            int                     path_cost) :
        node{ entry.p_key(),          entry.value(),       index_in_step, preceding_step,
                p_preceding_edge_costs, best_preceding_node, entry.cost(),  path_cost }
        {}
    */
    /*
        /*!
            \brief Returns true if one node is equal to another.

            \param one     One node.
            \param another Another node.

            \retval true  When one node is equal to another.
            \retval valse Otherwise.
        */
        friend constexpr bool operator==(const node& one, const node& another)
        {
            return ((!one.p_key() && !another.p_key()) ||
                    (one.p_key() && another.p_key() && *one.p_key() == *another.p_key())) &&
                    one.preceding_step() == another.preceding_step() &&
                    one.best_preceding_node() == another.best_preceding_node() &&
                    one.node_cost() == another.node_cost() && one.path_cost() == another.path_cost();
        }
    */
    /*
        /*!
            \brief Returns the key.

            \return The key.
        */
        [[nodiscard]] constexpr const input* p_key() const
        {
            return m_p_key;
        }
    */
    /*
        /*!
            \brief Returns the value.

            \return The value.
        */
        [[nodiscard]] constexpr const std::any& value() const
        {
            assert(m_p_value);
            return *m_p_value;
        }
    */
    /*
        /*!
            \brief Returns the index in the step.

            \return The index in the step.
        */
        [[nodiscard]] constexpr std::size_t index_in_step() const
        {
            return m_index_in_step;
        }
        /*!
            \brief Returns the index of the preceding step.

            \return The index of the preceding step.
        */
        [[nodiscard]] constexpr std::size_t preceding_step() const
        {
            return m_preceding_step;
        }
    */
    /*
        /*!
            \brief Returns the preceding edge costs.

            \return The preceding edge costs.
        */
        [[nodiscard]] constexpr const std::vector<int>& preceding_edge_costs() const
        {
            assert(m_p_preceding_edge_costs);
            return *m_p_preceding_edge_costs;
        }
    */
    /*
        /*!
            \brief Returns the index of the best preceding node.

            \return The index of the best preceding node.
        */
        [[nodiscard]] constexpr std::size_t best_preceding_node() const
        {
            return m_best_preceding_node;
        }
    */
    /*
        /*!
            \brief Returns the node cost.

            \return The node cost.
        */
        [[nodiscard]] constexpr int node_cost() const
        {
            return m_node_cost;
        }
    */
    /*
        /*!
            \brief Returns the path cost.

            \return The path cost.
        */
        [[nodiscard]] constexpr int path_cost() const
        {
            return m_path_cost;
        }
    */
    /*
        /*!
            \brief Returns true is this node is the BOS.

            \retval true  When this node is the BOS.
            \retval false Otherwise.
        */
        [[nodiscard]] bool is_bos() const;
    */
}

#[cfg(test)]
mod tests {
    use crate::StringInput;

    use super::*;

    #[test]
    fn bos() {
        let preceding_edge_costs = Vec::new();
        let _bos = Node::bos(&preceding_edge_costs);

        // BOOST_TEST(bos.p_key() == tetengo::lattice::entry_view::bos_eos().p_key());
        // BOOST_TEST(!bos.value().has_value());
        // BOOST_TEST(bos.preceding_step() == std::numeric_limits<std::size_t>::max());
        // BOOST_TEST(&bos.preceding_edge_costs() == &preceding_edge_costs);
        // BOOST_TEST(bos.best_preceding_node() == std::numeric_limits<std::size_t>::max());
        // BOOST_TEST(bos.node_cost() == tetengo::lattice::entry_view::bos_eos().cost());
        // BOOST_TEST(bos.path_cost() == 0);
    }

    #[test]
    fn eos() {
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let _eos = Node::eos(1, &preceding_edge_costs, 5, 42);

        // BOOST_TEST(eos.p_key() == tetengo::lattice::entry_view::bos_eos().p_key());
        // BOOST_TEST(!eos.value().has_value());
        // BOOST_TEST(eos.preceding_step() == 1U);
        // BOOST_TEST(&eos.preceding_edge_costs() == &preceding_edge_costs);
        // BOOST_TEST(eos.best_preceding_node() == 5U);
        // BOOST_TEST(eos.node_cost() == tetengo::lattice::entry_view::bos_eos().cost());
        // BOOST_TEST(eos.path_cost() == 42);
    }

    #[test]
    fn new() {
        {
            let key = StringInput::new(String::from("mizuho"));
            let value = 42;
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            let _node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);
        }
        // {
        //     let entry_key = StringInput::new(String::from("mizuho"));
        //     let entry_value = 42;
        //     let entry = EntryView::new(&entry_key, &entry_value, 24);
        //     let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        //     let _node = Node::new(&entry, 53, 1, &preceding_edge_costs, 5, 24, 2424);
        // }
    }
    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };
        }
        {
            const tetengo::lattice::string_input entry_key{ "mizuho" };
            const std::any                       entry_value{ 42 };
            const tetengo::lattice::entry_view   entry{ &entry_key, &entry_value, 24 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ entry, 53, 1, &preceding_edge_costs, 5, 2424 };

            BOOST_TEST(node_.p_key() == &entry_key);
            BOOST_TEST(std::any_cast<int>(node_.value()) == 42);
            BOOST_TEST(node_.preceding_step() == 1U);
            BOOST_TEST(&node_.preceding_edge_costs() == &preceding_edge_costs);
            BOOST_TEST(node_.best_preceding_node() == 5U);
            BOOST_TEST(node_.node_cost() == 24);
            BOOST_TEST(node_.path_cost() == 2424);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(operator_equal)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };

            const std::vector<int> preceding_edge_costs_bos{};
            const auto             bos = tetengo::lattice::node::bos(&preceding_edge_costs_bos);

            const std::vector<int> preceding_edge_costs_eos{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const auto             eos = tetengo::lattice::node::eos(1, &preceding_edge_costs_eos, 5, 42);

            const std::any               value1{ 42 };
            const std::vector<int>       preceding_edge_costs1{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node node1{ &key, &value1, 53, 1, &preceding_edge_costs1, 5, 24, 2424 };

            const std::any               value2{ 42 };
            const std::vector<int>       preceding_edge_costs2{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node node2{ &key, &value2, 53, 1, &preceding_edge_costs2, 5, 24, 2424 };

            BOOST_CHECK(bos == bos);
            BOOST_CHECK(bos != eos);
            BOOST_CHECK(bos != node1);
            BOOST_CHECK(node1 == node2);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(key)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST_REQUIRE(node_.p_key());
            BOOST_TEST_REQUIRE(node_.p_key()->is<tetengo::lattice::string_input>());
            BOOST_TEST(node_.p_key()->as<tetengo::lattice::string_input>().value() == "mizuho");
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(value)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(std::any_cast<int>(node_.value()) == 42);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(index_in_step)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(node_.index_in_step() == 53U);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(preceding_step)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(node_.preceding_step() == 1U);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(preceding_edge_costs)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(&node_.preceding_edge_costs() == &preceding_edge_costs);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(best_preceding_node)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(node_.best_preceding_node() == 5U);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(node_cost)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(node_.node_cost() == 24);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(path_cost)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo::lattice::node         node_{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 };

            BOOST_TEST(node_.path_cost() == 2424);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(is_bos)
    {
        BOOST_TEST_PASSPOINT();

        {
            const std::vector<int> preceding_edge_costs{};
            BOOST_TEST(tetengo::lattice::node::bos(&preceding_edge_costs).is_bos());
        }
        {
            const std::vector<int> preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            BOOST_TEST(!tetengo::lattice::node::eos(1, &preceding_edge_costs, 5, 42).is_bos());
        }
        {
            const tetengo::lattice::string_input key{ "mizuho" };
            const std::any                       value{ 42 };
            const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            BOOST_TEST((!tetengo::lattice::node{ &key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424 }.is_bos()));
        }
    }
    */
}
