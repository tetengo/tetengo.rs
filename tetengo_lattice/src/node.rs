/*!
 * A node.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use anyhow::Result;

use crate::entry::{AnyValue, EntryView};
use crate::input::Input;

/**
 * A node error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum NodeError {
    /**
     * A BOS or EOS entry is not allowed.
     */
    #[error("BOS or EOS entry is not allowed")]
    BosOrEosEntryNotAllowed,
}

/**
 * A BOS (Beginning of Sequence) node.
 */
#[derive(Clone, Copy, Debug)]
pub struct Bos<'a> {
    preceding_edge_costs: &'a Vec<i32>,
}

/**
 * A EOS (Ending of Sequence) node.
 */
#[derive(Clone, Copy, Debug)]
pub struct Eos<'a> {
    preceding_step: usize,
    preceding_edge_costs: &'a Vec<i32>,
    _best_preceding_node: usize,
    _path_cost: i32,
}
/**
 * A middle node.
 */
#[derive(Clone, Copy)]
pub struct Middle<'a> {
    key: &'a dyn Input,
    value: &'a dyn AnyValue,
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
#[derive(Clone, Debug /*, Eq*/)]
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
            preceding_edge_costs,
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
            preceding_step,
            preceding_edge_costs,
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
            key,
            value,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            node_cost,
            path_cost,
        })
    }

    /**
     * Creates a node from a vocabulary entry.
     *
     * # Errors
     * * When `entry` is BOS or EOS.
     */
    pub fn from(
        entry: &'a EntryView<'a>,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: &'a Vec<i32>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Result<Self> {
        let Some(key) = entry.key() else {
            return Err(NodeError::BosOrEosEntryNotAllowed.into());
        };
        let Some(value) = entry.value() else {
            return Err(NodeError::BosOrEosEntryNotAllowed.into());
        };
        Ok(Node::Middle(Middle {
            key,
            value,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            node_cost: entry.cost(),
            path_cost,
        }))
    }

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

    /**
     * Returns the key.
     *
     * # Returns
     * The key.
     */
    pub const fn key(&self) -> Option<&dyn Input> {
        match self {
            Node::Bos(_) => EntryView::BosEos.key(),
            Node::Eos(_) => EntryView::BosEos.key(),
            Node::Middle(middle) => Some(middle.key),
        }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub const fn value(&self) -> Option<&dyn AnyValue> {
        match self {
            Node::Bos(_) => EntryView::BosEos.value(),
            Node::Eos(_) => EntryView::BosEos.value(),
            Node::Middle(middle) => Some(middle.value),
        }
    }

    /**
     * Returns the index in the step.
     *
     * # Returns
     * The index in the step.
     */
    pub const fn index_in_step(&self) -> usize {
        match self {
            Node::Bos(_) => 0,
            Node::Eos(_) => 0,
            Node::Middle(middle) => middle.index_in_step,
        }
    }

    /**
     * Returns the preceding step.
     *
     * # Returns
     * The preceding step.
     */
    pub const fn preceding_step(&self) -> usize {
        match self {
            Node::Bos(_) => usize::MAX,
            Node::Eos(eos) => eos.preceding_step,
            Node::Middle(middle) => middle.preceding_step,
        }
    }

    /**
     * Returns the preceding edge costs.
     *
     * # Returns
     * The preceding edge costs.
     */
    pub const fn preceding_edge_costs(&self) -> &Vec<i32> {
        match self {
            Node::Bos(bos) => bos.preceding_edge_costs,
            Node::Eos(eos) => eos.preceding_edge_costs,
            Node::Middle(middle) => middle.preceding_edge_costs,
        }
    }
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

// impl PartialEq for Node<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Node::Bos(_), Node::Bos(_)) => true,
//             (Node::Eos(_), Node::Eos(_)) => true,
//             (Node::Middle(one), Node::Middle(another)) => {
//                 one.key == another.key
//                     && one.index_in_step == another.index_in_step
//                     && one.preceding_step == another.preceding_step
//                     && one.preceding_edge_costs == another.preceding_edge_costs
//                     && one.best_preceding_node == another.best_preceding_node
//                     && one.node_cost == another.node_cost
//                     && one.path_cost == another.path_cost
//             }
//             _ => false,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::StringInput;

    use super::*;

    #[test]
    fn bos() {
        let preceding_edge_costs = Vec::new();
        let bos = Node::bos(&preceding_edge_costs);

        assert!(bos.key().is_none());
        assert!(bos.value().is_none());
        assert_eq!(bos.index_in_step(), 0);
        assert_eq!(bos.preceding_step(), usize::MAX);
        assert_eq!(bos.preceding_edge_costs(), &preceding_edge_costs);
        // BOOST_TEST(bos.best_preceding_node() == std::numeric_limits<std::size_t>::max());
        // BOOST_TEST(bos.node_cost() == tetengo::lattice::entry_view::bos_eos().cost());
        // BOOST_TEST(bos.path_cost() == 0);
    }

    #[test]
    fn eos() {
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let eos = Node::eos(1, &preceding_edge_costs, 5, 42);

        assert!(eos.key().is_none());
        assert!(eos.value().is_none());
        assert_eq!(eos.index_in_step(), 0);
        assert_eq!(eos.preceding_step(), 1);
        assert_eq!(eos.preceding_edge_costs(), &preceding_edge_costs);
        // BOOST_TEST(eos.best_preceding_node() == 5U);
        // BOOST_TEST(eos.node_cost() == tetengo::lattice::entry_view::bos_eos().cost());
        // BOOST_TEST(eos.path_cost() == 42);
    }

    #[test]
    fn new() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let _node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);
    }

    #[test]
    fn from() {
        {
            let entry_key = StringInput::new(String::from("mizuho"));
            let entry_value = 42;
            let entry = EntryView::new(&entry_key, &entry_value, 24);
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            let node = Node::from(&entry, 53, 1, &preceding_edge_costs, 5, 2424);

            let node = node.unwrap();
            assert_eq!(
                node.key()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap(),
                &entry_key
            );
            assert_eq!(
                node.value()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<i32>()
                    .unwrap(),
                &42
            );
            assert_eq!(node.index_in_step(), 53);
            assert_eq!(node.preceding_step(), 1);
            assert_eq!(node.preceding_edge_costs(), &preceding_edge_costs);
            // BOOST_TEST(node_.best_preceding_node() == 5U);
            // BOOST_TEST(node_.node_cost() == 24);
            // BOOST_TEST(node_.path_cost() == 2424);
        }
        {
            let entry = EntryView::BosEos;
            let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
            let node = Node::from(&entry, 53, 1, &preceding_edge_costs, 5, 2424);

            assert!(node.is_err());
        }
    }

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

    #[test]
    fn key() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(
            node.key()
                .unwrap()
                .as_any()
                .downcast_ref::<StringInput>()
                .unwrap()
                .value(),
            "mizuho"
        );
    }

    #[test]
    fn value() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(
            node.value()
                .unwrap()
                .as_any()
                .downcast_ref::<i32>()
                .unwrap(),
            &42
        );
    }

    #[test]
    fn index_in_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.index_in_step(), 53);
    }

    #[test]
    fn preceding_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.preceding_step(), 1);
    }

    #[test]
    fn preceding_edge_costs() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let node = Node::new(&key, &value, 53, 1, &preceding_edge_costs, 5, 24, 2424);

        assert_eq!(node.preceding_edge_costs(), &preceding_edge_costs);
    }
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
