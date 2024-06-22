/*!
 * A node constraint element.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

/**
 * A node constraint element.
 */
#[derive(Copy, Clone, Debug)]
pub struct NodeConstraintElement {
    /*
        private:
            // variables

            const node m_node;
    */
}

impl NodeConstraintElement {
    /*
           /*!
               \brief Creates a node constraint element.

               \param node_ A node.
           */
           explicit node_constraint_element(node node_);
    */
    /*
        // constructors and destructor

        explicit impl(node node_) : m_node{ std::move(node_) } {}
    */
    /*
            int matches_impl(const node& node_) const
            {
                return node_ == m_node ? 0 : -1;
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
            const tetengo::lattice::string_input element_node_key{ "mizuho" };
            const std::any                       element_node_value{ 42 };
            const std::vector<int>               element_node_preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            tetengo::lattice::node               element_node{
                &element_node_key, &element_node_value, 0, 1, &element_node_preceding_edge_costs, 5, 24, 2424
            };
            const tetengo::lattice::node_constraint_element element{ std::move(element_node) };
        }

        {
            const auto* const p_element_key = tetengo_lattice_input_createStringInput("mizuho");
            BOOST_SCOPE_EXIT(p_element_key)
            {
                tetengo_lattice_input_destroy(p_element_key);
            }
            BOOST_SCOPE_EXIT_END;
            const std::any               element_value{ reinterpret_cast<const void*>("MIZUHO") };
            const std::vector<int>       element_preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo_lattice_node_t element_node{ tetengo_lattice_entryView_toKeyHandle(p_element_key),
                                                       reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(
                                                           &element_value),
                                                       0,
                                                       1,
                                                       std::data(element_preceding_edge_costs),
                                                       std::size(element_preceding_edge_costs),
                                                       5,
                                                       24,
                                                       2424 };
            const auto* const            p_constraint_element =
                tetengo_lattice_constraintElement_createNodeConstraintElement(&element_node);
            BOOST_SCOPE_EXIT(p_constraint_element)
            {
                tetengo_lattice_constraintElement_destroy(p_constraint_element);
            }
            BOOST_SCOPE_EXIT_END;
            BOOST_TEST(p_constraint_element);
        }
        {
            const auto* const p_constraint_element = tetengo_lattice_constraintElement_createNodeConstraintElement(nullptr);
            BOOST_TEST(!p_constraint_element);
        }
    }
    */
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

        {
            const auto* const p_element_key = tetengo_lattice_input_createStringInput("mizuho");
            BOOST_SCOPE_EXIT(p_element_key)
            {
                tetengo_lattice_input_destroy(p_element_key);
            }
            BOOST_SCOPE_EXIT_END;
            const std::any               element_value{ reinterpret_cast<const void*>("MIZUHO") };
            const std::vector<int>       element_preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo_lattice_node_t element_node{ tetengo_lattice_entryView_toKeyHandle(p_element_key),
                                                       reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(
                                                           &element_value),
                                                       0,
                                                       1,
                                                       std::data(element_preceding_edge_costs),
                                                       std::size(element_preceding_edge_costs),
                                                       5,
                                                       24,
                                                       2424 };
            const auto* const            p_constraint_element =
                tetengo_lattice_constraintElement_createNodeConstraintElement(&element_node);
            BOOST_SCOPE_EXIT(p_constraint_element)
            {
                tetengo_lattice_constraintElement_destroy(p_constraint_element);
            }
            BOOST_SCOPE_EXIT_END;
            BOOST_TEST_REQUIRE(p_constraint_element);

            {
                const auto* const p_key = tetengo_lattice_input_createStringInput("mizuho");
                BOOST_SCOPE_EXIT(p_key)
                {
                    tetengo_lattice_input_destroy(p_key);
                }
                BOOST_SCOPE_EXIT_END;
                const std::any               value{ reinterpret_cast<const void*>("MIZUHO") };
                const std::vector<int>       preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo_lattice_node_t node{ tetengo_lattice_entryView_toKeyHandle(p_key),
                                                   reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(&value),
                                                   0,
                                                   1,
                                                   std::data(preceding_edge_costs),
                                                   std::size(preceding_edge_costs),
                                                   5,
                                                   24,
                                                   2424 };

                BOOST_TEST(tetengo_lattice_constraintElement_matches(p_constraint_element, &node) == 0);
            }
            {
                const auto* const p_key = tetengo_lattice_input_createStringInput("sakura");
                BOOST_SCOPE_EXIT(p_key)
                {
                    tetengo_lattice_input_destroy(p_key);
                }
                BOOST_SCOPE_EXIT_END;
                const std::any               value{ reinterpret_cast<const void*>("SAKURA") };
                const std::vector<int>       preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
                const tetengo_lattice_node_t node{ tetengo_lattice_entryView_toKeyHandle(p_key),
                                                   reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(&value),
                                                   0,
                                                   1,
                                                   std::data(preceding_edge_costs),
                                                   std::size(preceding_edge_costs),
                                                   5,
                                                   24,
                                                   2424 };

                BOOST_TEST(tetengo_lattice_constraintElement_matches(p_constraint_element, &node) < 0);
            }
        }
        {
            const auto* const p_key = tetengo_lattice_input_createStringInput("mizuho");
            BOOST_SCOPE_EXIT(p_key)
            {
                tetengo_lattice_input_destroy(p_key);
            }
            BOOST_SCOPE_EXIT_END;
            const std::any               value{ reinterpret_cast<const void*>("MIZUHO") };
            const std::vector<int>       preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo_lattice_node_t node{ tetengo_lattice_entryView_toKeyHandle(p_key),
                                               reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(&value),
                                               0,
                                               1,
                                               std::data(preceding_edge_costs),
                                               std::size(preceding_edge_costs),
                                               5,
                                               24,
                                               2424 };

            BOOST_TEST(tetengo_lattice_constraintElement_matches(nullptr, &node) < 0);
        }
        {
            const auto* const p_element_key = tetengo_lattice_input_createStringInput("mizuho");
            BOOST_SCOPE_EXIT(p_element_key)
            {
                tetengo_lattice_input_destroy(p_element_key);
            }
            BOOST_SCOPE_EXIT_END;
            const std::any               element_value{ reinterpret_cast<const void*>("MIZUHO") };
            const std::vector<int>       element_preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            const tetengo_lattice_node_t element_node{ tetengo_lattice_entryView_toKeyHandle(p_element_key),
                                                       reinterpret_cast<tetengo_lattice_entryView_valueHandle_t>(
                                                           &element_value),
                                                       0,
                                                       1,
                                                       std::data(element_preceding_edge_costs),
                                                       std::size(element_preceding_edge_costs),
                                                       5,
                                                       24,
                                                       2424 };
            const auto* const            p_constraint_element =
                tetengo_lattice_constraintElement_createNodeConstraintElement(&element_node);
            BOOST_SCOPE_EXIT(p_constraint_element)
            {
                tetengo_lattice_constraintElement_destroy(p_constraint_element);
            }
            BOOST_SCOPE_EXIT_END;
            BOOST_TEST_REQUIRE(p_constraint_element);

            BOOST_TEST(tetengo_lattice_constraintElement_matches(p_constraint_element, nullptr) < 0);
        }
    }
     */
}
