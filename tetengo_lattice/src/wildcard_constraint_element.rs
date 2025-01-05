/*!
 * A wildcard constraint element.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use crate::constraint_element::ConstraintElement;
use crate::node::Node;

/**
 * A wildcard constraint element.
 */
#[derive(Clone, Copy, Debug)]
pub struct WildcardConstraintElement {
    preceding_step: usize,
}

impl WildcardConstraintElement {
    /**
     * Creates a wildcard constraint element.
     *
     * # Arguments
     * * `preceding_step` - An index of a preceding step.
     */
    pub const fn new(preceding_step: usize) -> Self {
        Self { preceding_step }
    }
}

impl ConstraintElement for WildcardConstraintElement {
    fn matches(&self, node: &Node) -> i32 {
        if self.preceding_step == usize::MAX {
            if node.preceding_step() == usize::MAX {
                0
            } else {
                1
            }
        } else if node.preceding_step() < self.preceding_step {
            -1
        } else {
            (node.preceding_step() - self.preceding_step) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::string_input::StringInput;

    use super::*;

    #[test]
    const fn new() {
        let _ = WildcardConstraintElement::new(3);
    }

    #[test]
    fn matches() {
        {
            let element = WildcardConstraintElement::new(3);

            {
                let key = StringInput::new(String::from("mizuho"));
                let value = 42;
                let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
                let node = Node::new(
                    Box::new(key),
                    Box::new(value),
                    0,
                    1,
                    preceding_edge_costs,
                    5,
                    24,
                    2424,
                );

                assert!(element.matches(&node) < 0);
            }
            {
                let key = StringInput::new(String::from("sakura"));
                let value = 42;
                let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
                let node = Node::new(
                    Box::new(key),
                    Box::new(value),
                    0,
                    3,
                    preceding_edge_costs,
                    5,
                    24,
                    2424,
                );

                assert_eq!(element.matches(&node), 0);
            }
            {
                let key = StringInput::new(String::from("tsubame"));
                let value = 42;
                let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
                let node = Node::new(
                    Box::new(key),
                    Box::new(value),
                    0,
                    5,
                    preceding_edge_costs,
                    5,
                    24,
                    2424,
                );

                assert!(element.matches(&node) > 0);
            }
        }
        {
            let element = WildcardConstraintElement::new(usize::MAX);

            {
                let preceding_edge_costs = Rc::new(Vec::new());
                let node = Node::bos(preceding_edge_costs);

                assert_eq!(element.matches(&node), 0);
            }
            {
                let key = StringInput::new(String::from("mizuho"));
                let value = 42;
                let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
                let node = Node::new(
                    Box::new(key),
                    Box::new(value),
                    0,
                    1,
                    preceding_edge_costs,
                    5,
                    24,
                    2424,
                );

                assert!(element.matches(&node) > 0);
            }
        }
    }
}
