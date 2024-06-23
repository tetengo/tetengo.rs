/*!
 * A wildcard constraint element.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

/**
 * A wildcard constraint element.
 */
#[derive(Copy, Clone, Debug)]
pub struct WildcardConstraintElement {
    /*
       private:
           // variables

           const std::size_t m_preceding_step;
    */
}

impl WildcardConstraintElement {
    /*
           /*!
               \brief Creates a wildcard constraint element.

               \param preceding_step An index of a preceding step.
           */
           explicit wildcard_constraint_element(std::size_t preceding_step);
    */
    /*
           explicit impl(const std::size_t preceding_step) : m_preceding_step{ preceding_step } {}
    */
    /*
           // functions

           int matches_impl(const node& node_) const
           {
               if (m_preceding_step == std::numeric_limits<std::size_t>::max())
               {
                   if (node_.preceding_step() == std::numeric_limits<std::size_t>::max())
                   {
                       return 0;
                   }
                   else
                   {
                       return 1;
                   }
               }
               else
               {
                   if (node_.preceding_step() < m_preceding_step)
                   {
                       return -1;
                   }
                   else
                   {
                       return static_cast<int>(node_.preceding_step() - m_preceding_step);
                   }
               }
           }
    */
}

#[cfg(test)]
mod tests {
    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::wildcard_constraint_element element{ 3 };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(matches)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::wildcard_constraint_element element{ 3 };

            {
                const tetengo::lattice::string_input key{ "mizuho" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 1, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) < 0);
            }
            {
                const tetengo::lattice::string_input key{ "sakura" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 3, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) == 0);
            }
            {
                const tetengo::lattice::string_input key{ "tsubame" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 5, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) > 0);
            }
        }
        {
            const tetengo::lattice::wildcard_constraint_element element{ std::numeric_limits<std::size_t>::max() };

            {
                const std::vector<int>       preceding_edge_costs{};
                const tetengo::lattice::node node_ = tetengo::lattice::node::bos(&preceding_edge_costs);

                BOOST_TEST(element.matches(node_) == 0);
            }
            {
                const tetengo::lattice::string_input key{ "mizuho" };
                const std::any                       value{ 42 };
                const std::vector<int>               preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo::lattice::node         node_{ &key, &value, 0, 1, &preceding_edge_costs, 5, 24, 2424 };

                BOOST_TEST(element.matches(node_) > 0);
            }
        }
    }
     */
}
