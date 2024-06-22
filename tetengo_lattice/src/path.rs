/*!
 * A path.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::node::Node;

/**
 * A path.
 */
#[derive(Debug, Default)]
pub struct Path<'a> {
    nodes: Vec<Node<'a>>,
    _cost: i32,
}

impl<'a> Path<'a> {
    /**
     * Creates an empty path.
     */
    pub fn new() -> Self {
        Path {
            nodes: Vec::new(),
            _cost: 0,
        }
    }

    /**
     * Creates a path.
     *
     * # Arguments
     * * `nodes` - Nodes.
     * * `cost`  - A cost.
     */
    pub fn new_with_nodes(nodes: Vec<Node<'a>>, cost: i32) -> Self {
        Path { nodes, _cost: cost }
    }

    /**
     * Returns `true` if this path is empty.
     *
     * # Returns
     * `true` if this path is empty.
     */
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /*
           /*!
               \brief Returns the nodes.

               \return The nodes.
           */
           [[nodiscard]] const std::vector<node>& nodes() const;
    */
    /*
       const std::vector<node>& path::nodes() const
       {
           return m_nodes;
       }
    */
    /*
           /*!
               \brief Returns the cost.

               \return The cost.
           */
           [[nodiscard]] int cost() const;
    */
    /*
       int path::cost() const
       {
           return m_cost;
       }
    */
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::string_input::StringInput;
    use once_cell::sync::Lazy;

    use super::*;

    const NODE_VALUE: i32 = 42;

    const BOS_PRECEDING_EDGE_COSTS: Vec<i32> = vec![];

    static PRECEDING_EDGE_COSTS: Lazy<Vec<i32>> = Lazy::new(|| vec![1]);

    fn nodes() -> Vec<Node<'static>> {
        static KEY_MIZUHO: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("mizuho")));
        static KEY_SAKURA: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("sakura")));
        static KEY_TSUBAME: Lazy<StringInput> =
            Lazy::new(|| StringInput::new(String::from("tsubame")));
        vec![
            Node::bos(Rc::new(BOS_PRECEDING_EDGE_COSTS)),
            Node::new(
                &*KEY_MIZUHO,
                &NODE_VALUE,
                0,
                0,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_SAKURA,
                &NODE_VALUE,
                0,
                1,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::new(
                &*KEY_TSUBAME,
                &NODE_VALUE,
                0,
                2,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::eos(3, Rc::new(PRECEDING_EDGE_COSTS.clone()), 0, 0),
        ]
    }

    /*
        bool equal_nodes(const std::vector<tetengo_lattice_node_t>& one, const std::vector<tetengo_lattice_node_t>& another)
        {
            if (std::size(one) != std::size(another))
            {
                return false;
            }

            for (auto i = static_cast<std::size_t>(0); i < std::size(one); ++i)
            {
                if (!tetengo_lattice_node_equal(&one[i], &another[i]))
                {
                    return false;
                }
            }

            return true;
        }


    }
    */

    #[test]
    fn new() {
        let _path = Path::new();
    }

    #[test]
    fn new_with_nodes() {
        let _path = Path::new_with_nodes(nodes(), 42);
    }

    #[test]
    fn is_empty() {
        {
            let path = Path::new();
            assert!(path.is_empty());
        }
        {
            let path = Path::new_with_nodes(nodes(), 42);
            assert!(!path.is_empty());
        }
    }

    /*
    BOOST_AUTO_TEST_CASE(nodes)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::path path_{};

            BOOST_TEST(std::empty(path_.nodes()));
        }
        {
            const tetengo::lattice::path path_{ cpp_nodes(), 42 };

            BOOST_CHECK(path_.nodes() == cpp_nodes());
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(cost)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::path path_{};

            BOOST_TEST(path_.cost() == 0);
        }
        {
            const tetengo::lattice::path path_{ cpp_nodes(), 42 };

            BOOST_TEST(path_.cost() == 42);
        }
    }
         */
}
