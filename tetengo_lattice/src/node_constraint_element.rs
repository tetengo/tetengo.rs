/*!
 * A node constraint element.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::node::Node;

/**
 * A node constraint element.
 */
#[derive(Clone, Debug)]
pub struct NodeConstraintElement<'a> {
    _node: Node<'a>,
}

impl<'a> NodeConstraintElement<'a> {
    /**
     * Creates a node constraint element.
     *
     * # Arguments
     * * `node` - A node.
     */
    pub fn new(node: Node<'a>) -> Self {
        Self { _node: node }
    }

    /*
            int matches_impl(const node& node_) const
            {
                return node_ == m_node ? 0 : -1;
            }
    */
}
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::string_input::StringInput;

    use super::*;

    #[test]
    fn new() {
        let element_node_key = StringInput::new(String::from("mizuho"));
        let element_node_value = 42;
        let element_node_preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let element_node = Node::new(
            &element_node_key,
            &element_node_value,
            0,
            1,
            element_node_preceding_edge_costs,
            5,
            24,
            2424,
        );
        let _element = NodeConstraintElement::new(element_node);
    }

    /*
    BOOST_AUTO_TEST_CASE(matches)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input element_node_key{ "mizuho" };
            const std::any                       element_node_value{ 42 };
            const std::vector<int>               element_node_preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            tetengo::lattice::node               element_node{
                &element_node_key, &element_node_value, 0, 1, &element_node_preceding_edge_costs, 5, 24, 2424
            };
            const tetengo::lattice::node_constraint_element element{ std::move(element_node) };

            {
                const tetengo::lattice::string_input key{ "mizuho" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 1, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) == 0);
            }
            {
                const tetengo::lattice::string_input key{ "sakura" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 1, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) < 0);
            }
        }

    }
     */
}
