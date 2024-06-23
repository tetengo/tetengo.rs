/*!
 * A node constraint element.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::constraint_element::ConstraintElement;
use crate::node::Node;

/**
 * A node constraint element.
 */
#[derive(Clone, Debug)]
pub struct NodeConstraintElement<'a> {
    node: Node<'a>,
}

impl<'a> NodeConstraintElement<'a> {
    /**
     * Creates a node constraint element.
     *
     * # Arguments
     * * `node` - A node.
     */
    pub fn new(node: Node<'a>) -> Self {
        Self { node }
    }
}

impl ConstraintElement for NodeConstraintElement<'_> {
    fn matches(&self, node: &Node<'_>) -> i32 {
        if *node == self.node {
            0
        } else {
            -1
        }
    }
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

    #[test]
    fn matches() {
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
        let element = NodeConstraintElement::new(element_node);

        {
            let key = StringInput::new(String::from("mizuho"));
            let value = 42;
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::new(&key, &value, 0, 1, preceding_edge_costs, 5, 24, 2424);

            assert_eq!(element.matches(&node), 0);
        }
        {
            let key = StringInput::new(String::from("sakura"));
            let value = 42;
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::new(&key, &value, 0, 1, preceding_edge_costs, 5, 24, 2424);

            assert!(element.matches(&node) < 0);
        }
    }
}
