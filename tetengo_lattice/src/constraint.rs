/*!
 * A constraint.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::constraint_element::ConstraintElement;
use crate::Node;

/**
 * A constraint.
 */
#[derive(Debug, Default)]
pub struct Constraint<'a> {
    pattern: Vec<Box<dyn ConstraintElement + 'a>>,
}

impl<'a> Constraint<'a> {
    /**
     * Creates an empty constraint.
     *
     * It matches any path.
     */
    pub const fn new() -> Self {
        Self {
            pattern: Vec::new(),
        }
    }

    /**
     * Creates a constraint.
     *
     * # Arguments
     * * `pattern` - A pattern.
     */
    pub const fn new_with_pattern(pattern: Vec<Box<dyn ConstraintElement + 'a>>) -> Self {
        Self { pattern }
    }

    /**
     * Returns `true`` if the path matches the pattern.
     *
     * # Arguments
     * * `reverse_path` - A path in reverse order.
     *
     * # Returns
     * `true` if the path matches the pattern.
     */
    pub fn matches(&self, reverse_path: &[Node<'_>]) -> bool {
        self.matches_impl(reverse_path) == 0
    }

    /**
     * Returns `true`` if the tail path matches the tail of the pattern.
     *
     * # Arguments
     * * `reverse_tail_path` - A tail path in reverse order.
     *
     * # Returns
     * `true` if the tail path matches the tail of the pattern.
     */
    pub fn matches_tail(&self, reverse_tail_path: &[Node<'_>]) -> bool {
        self.matches_impl(reverse_tail_path) != usize::MAX
    }

    fn matches_impl(&self, reverse_path: &[Node<'_>]) -> usize {
        if self.pattern.is_empty() {
            return 0;
        }

        let mut pattern_index = self.pattern.len();
        for node in reverse_path {
            if pattern_index == 0 {
                break;
            }

            let element_match = self.pattern[pattern_index - 1].matches(node);
            match element_match {
                m if m < 0 => return usize::MAX,
                0 => {
                    if pattern_index == 0 {
                        return usize::MAX;
                    }
                    pattern_index -= 1;
                }
                _ => {}
            }
        }

        pattern_index
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use once_cell::sync::Lazy;

    use crate::node_constraint_element::NodeConstraintElement;
    use crate::string_input::StringInput;
    use crate::wildcard_constraint_element::WildcardConstraintElement;

    use super::*;

    const NODE_VALUE: i32 = 42;

    fn bos_preceding_edge_costs() -> Rc<Vec<i32>> {
        Rc::new(Vec::new())
    }

    fn preceding_edge_costs() -> Rc<Vec<i32>> {
        Rc::new(vec![1])
    }

    fn make_path_b_e() -> Vec<Node<'static>> {
        vec![
            Node::bos(bos_preceding_edge_costs()),
            Node::eos(0, preceding_edge_costs(), 0, 0),
        ]
    }

    fn make_path_b_m_s_t_e() -> Vec<Node<'static>> {
        static KEY_MIZUHO: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("mizuho")));
        static KEY_SAKURA: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("sakura")));
        static KEY_TSUBAME: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("tsubame")));
        vec![
            Node::bos(bos_preceding_edge_costs()),
            Node::new(
                &*KEY_MIZUHO,
                &NODE_VALUE,
                0,
                0,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_SAKURA,
                &NODE_VALUE,
                0,
                1,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_TSUBAME,
                &NODE_VALUE,
                0,
                2,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::eos(3, preceding_edge_costs(), 0, 0),
        ]
    }

    fn make_path_b_m_a_t_e() -> Vec<Node<'static>> {
        static KEY_MIZUHO: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("mizuho")));
        static KEY_ARIAKE: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("ariake")));
        static KEY_TSUBAME: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("tsubame")));
        vec![
            Node::bos(bos_preceding_edge_costs()),
            Node::new(
                &*KEY_MIZUHO,
                &NODE_VALUE,
                0,
                0,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_ARIAKE,
                &NODE_VALUE,
                0,
                1,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_TSUBAME,
                &NODE_VALUE,
                0,
                2,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::eos(3, preceding_edge_costs(), 0, 0),
        ]
    }

    fn make_path_b_h_t_e() -> Vec<Node<'static>> {
        static KEY_HINOKUNI: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("hinokuni")));
        static KEY_TSUBAME: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("tsubame")));
        vec![
            Node::bos(bos_preceding_edge_costs()),
            Node::new(
                &*KEY_HINOKUNI,
                &NODE_VALUE,
                0,
                0,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_TSUBAME,
                &NODE_VALUE,
                0,
                2,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::eos(3, preceding_edge_costs(), 0, 0),
        ]
    }

    fn make_path_b_k_s_k_e() -> Vec<Node<'static>> {
        static KEY_KAMOME: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("kamome")));
        static KEY_SAKURA: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("sakura")));
        static KEY_KUMAGAWA: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("kumagawa")));
        vec![
            Node::bos(bos_preceding_edge_costs()),
            Node::new(
                &*KEY_KAMOME,
                &NODE_VALUE,
                0,
                0,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_SAKURA,
                &NODE_VALUE,
                0,
                1,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_KUMAGAWA,
                &NODE_VALUE,
                0,
                2,
                preceding_edge_costs(),
                0,
                0,
                0,
            ),
            Node::eos(3, preceding_edge_costs(), 0, 0),
        ]
    }

    fn reverse_path(path: Vec<Node<'_>>) -> Vec<Node<'_>> {
        path.into_iter().rev().collect()
    }

    fn make_pattern_b_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(NodeConstraintElement::new(path[1].clone())),
        ]
    }

    fn make_pattern_b_m_s_t_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_m_s_t_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(NodeConstraintElement::new(path[1].clone())),
            Box::new(NodeConstraintElement::new(path[2].clone())),
            Box::new(NodeConstraintElement::new(path[3].clone())),
            Box::new(NodeConstraintElement::new(path[4].clone())),
        ]
    }

    fn make_pattern_b_m_w_t_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_m_s_t_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(NodeConstraintElement::new(path[1].clone())),
            Box::new(WildcardConstraintElement::new(1)),
            Box::new(NodeConstraintElement::new(path[3].clone())),
            Box::new(NodeConstraintElement::new(path[4].clone())),
        ]
    }

    fn make_pattern_b_w_t_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_m_s_t_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(WildcardConstraintElement::new(0)),
            Box::new(NodeConstraintElement::new(path[3].clone())),
            Box::new(NodeConstraintElement::new(path[4].clone())),
        ]
    }

    fn make_pattern_b_w_s_w_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_m_s_t_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(WildcardConstraintElement::new(0)),
            Box::new(NodeConstraintElement::new(path[2].clone())),
            Box::new(WildcardConstraintElement::new(2)),
            Box::new(NodeConstraintElement::new(path[4].clone())),
        ]
    }

    fn make_pattern_b_w_e() -> Vec<Box<dyn ConstraintElement>> {
        let path = make_path_b_m_s_t_e();
        vec![
            Box::new(NodeConstraintElement::new(path[0].clone())),
            Box::new(WildcardConstraintElement::new(0)),
            Box::new(NodeConstraintElement::new(path[4].clone())),
        ]
    }

    fn make_pattern_w() -> Vec<Box<dyn ConstraintElement>> {
        vec![Box::new(WildcardConstraintElement::new(usize::MAX))]
    }

    fn make_tail(path: Vec<Node<'_>>, node_count: usize) -> Vec<Node<'_>> {
        assert!(0 < node_count && node_count <= path.len());
        path[path.len() - node_count..].to_vec()
    }

    #[test]
    fn new() {
        let _constraint = Constraint::new();
    }

    #[test]
    fn new_with_pattern() {
        let _constraint = Constraint::new_with_pattern(make_pattern_b_e());
    }

    #[test]
    fn matches() {
        {
            let constraint = Constraint::new();

            assert!(constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_e());

            assert!(constraint.matches(&reverse_path(make_path_b_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_m_s_t_e());

            assert!(!constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_m_w_t_e());

            assert!(!constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_t_e());

            assert!(!constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_s_w_e());

            assert!(!constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(!constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_e());

            assert!(!constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_w());

            assert!(constraint.matches(&reverse_path(make_path_b_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_s_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_m_a_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_h_t_e())));
            assert!(constraint.matches(&reverse_path(make_path_b_k_s_k_e())));
        }
    }

    #[test]
    fn matches_tail() {
        {
            let constraint = Constraint::new();

            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 5))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_e());

            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_m_s_t_e());

            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_m_w_t_e());

            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_t_e());

            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_s_w_e());

            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 5))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_b_w_e());

            assert!(!constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 5))));
        }
        {
            let constraint = Constraint::new_with_pattern(make_pattern_w());

            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_s_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_m_a_t_e(), 5))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_h_t_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 1))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 2))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 3))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 4))));
            assert!(constraint.matches_tail(&reverse_path(make_tail(make_path_b_k_s_k_e(), 5))));
        }
    }
}
