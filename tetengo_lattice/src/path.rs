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
    _nodes: Vec<Node<'a>>,
    _cost: i32,
}

impl Path<'_> {
    /**
     * Creates an empty path.
     */
    pub fn new() -> Self {
        Path {
            _nodes: Vec::new(),
            _cost: 0,
        }
    }

    /*
           /*!
               \brief Creates a path.

               \param nodes Nodes.
               \param cost  A cost.
           */
           path(std::vector<node> nodes, int cost);
    */
    /*
       path::path(std::vector<node> nodes, const int cost) : m_nodes{ std::move(nodes) }, m_cost{ cost } {}
    */
    /*
           // functions

           /*!
               \brief Returns true when this path is empty.

               \retval true  When this path is empty.
               \retval false Otherwise.
           */
           [[nodiscard]] bool empty() const;
    */
    /*
       bool path::empty() const
       {
           return std::empty(m_nodes);
       }
    */
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
    use super::*;

    /*
    namespace
    {
        const std::any& node_value()
        {
            static const std::any singleton{ 42 };
            return singleton;
        }
    */
    /*
        const std::vector<int>& bos_preceding_edge_costs()
        {
            static const std::vector<int> singleton{};
            return singleton;
        }
    */
    /*
        const std::vector<int>& preceding_edge_costs()
        {
            static const std::vector<int> singleton{ 1 };
            return singleton;
        }
    */
    /*
        const std::vector<tetengo::lattice::node>& cpp_nodes()
        {
            static const tetengo::lattice::string_input      key_mizuho{ "mizuho" };
            static const tetengo::lattice::string_input      key_sakura{ "sakura" };
            static const tetengo::lattice::string_input      key_tsubame{ "tsubame" };
            static const std::vector<tetengo::lattice::node> singleton{
                tetengo::lattice::node::bos(&bos_preceding_edge_costs()),
                tetengo::lattice::node{ &key_mizuho, &node_value(), 0, 0, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_sakura, &node_value(), 0, 1, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_tsubame, &node_value(), 0, 2, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node::eos(3, &preceding_edge_costs(), 0, 0)
            };
            return singleton;
        }
    */
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
    /*
    BOOST_AUTO_TEST_SUITE(test_tetengo)
    BOOST_AUTO_TEST_SUITE(lattice)
    BOOST_AUTO_TEST_SUITE(path)


    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::path path_{};
        }
        {
            const tetengo::lattice::path path_{ cpp_nodes(), 42 };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(empty)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::path path_{};

            BOOST_TEST(std::empty(path_));
        }
        {
            const tetengo::lattice::path path_{ cpp_nodes(), 42 };

            BOOST_TEST(!std::empty(path_));
        }
    }
    */
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
