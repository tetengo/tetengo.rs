/*!
 * A constraint.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::constraint_element::ConstraintElement;
use crate::Node;

/**
 * A constraint.
 */
#[derive(Default)]
pub struct Constraint {
    pattern: Vec<Box<dyn ConstraintElement>>,
}

impl Constraint {
    /**
     * Creates an empty constraint.
     *
     * It matches any path.
     */
    pub fn new() -> Self {
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
    pub fn new_with_pattern(pattern: Vec<Box<dyn ConstraintElement>>) -> Self {
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

    /*
           /*!
               \brief Returns true when the tail path matches the tail of the pattern.

               \param reverse_tail_path A tail path in reverse order.

               \retval true  When the tail path matches the tail of the pattern.
               \retval false Otherwise.
           */
           [[nodiscard]] bool matches_tail(const std::vector<node>& reverse_tail_path) const;
    */
    /*
           bool matches_tail(const std::vector<node>& reverse_tail_path) const
           {
               return matches_impl(reverse_tail_path) != std::numeric_limits<std::size_t>::max();
           }
    */

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

impl Debug for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Constraint")
            .field("pattern", &"Vec<Box<dyn ConstraintElement>>")
            .finish()
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
            Node::eos(0, preceding_edge_costs(), 0, 0),
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
            Node::eos(0, preceding_edge_costs(), 0, 0),
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
            Node::eos(0, preceding_edge_costs(), 0, 0),
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
            Node::eos(0, preceding_edge_costs(), 0, 0),
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

    /*
        std::vector<tetengo::lattice::node>
        make_tail(const std::vector<tetengo::lattice::node>& path, const std::size_t node_count)
        {
            assert(0 < node_count && node_count <= std::size(path));
            return std::vector<tetengo::lattice::node>{ std::next(std::begin(path), std::size(path) - node_count),
                                                        std::end(path) };
        }


    }
    */

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

    /*
    BOOST_AUTO_TEST_CASE(matches_tail_cpp)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::constraint constraint_{};

            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 5))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_e() };

            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_m_s_t_e() };

            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_m_w_t_e() };

            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_t_e() };

            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_s_w_e() };

            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 5))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_e() };

            BOOST_TEST(!constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 5))));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_w() };

            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_s_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_m_a_t_e(), 5))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_h_t_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 1))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 2))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 3))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 4))));
            BOOST_TEST(constraint_.matches_tail(reverse_path(make_tail(path_b_k_s_k_e(), 5))));
        }
    }

    */
}
