/*!
 * A constraint.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::constraint_element::ConstraintElement;

/**
 * A constraint.
 */
pub struct Constraint {
    _pattern: Vec<Box<dyn ConstraintElement>>,
}

impl Constraint {
    /*
           /*!
               \brief Creates an empty constraint.

               It matches any path.
           */
           constraint();
    */
    /*
           /*!
               \brief Creates a constraint.

               \param pattern A pattern.
           */
           explicit constraint(std::vector<std::unique_ptr<constraint_element>>&& pattern);
    */
    /*
           // functions

           /*!
               \brief Returns true when the path matches the pattern.

               \param reverse_path A path in reverse order.

               \retval true  When the path matches the pattern.
               \retval false Otherwise.
           */
           [[nodiscard]] bool matches(const std::vector<node>& reverse_path) const;
    */
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
           impl() : m_pattern{} {}
    */
    /*
           explicit impl(std::vector<std::unique_ptr<constraint_element>>&& pattern) : m_pattern{ std::move(pattern) } {}
    */
    /*
           // functions

           bool matches(const std::vector<node>& reverse_path) const
           {
               return matches_impl(reverse_path) == 0;
           }
    */
    /*
           bool matches_tail(const std::vector<node>& reverse_tail_path) const
           {
               return matches_impl(reverse_tail_path) != std::numeric_limits<std::size_t>::max();
           }
    */
    /*
           // functions

           std::size_t matches_impl(const std::vector<node>& reverse_path) const
           {
               if (std::empty(m_pattern))
               {
                   return 0;
               }

               auto pattern_index = std::size(m_pattern);
               for (auto path_index = static_cast<std::size_t>(0);
                    path_index < std::size(reverse_path) && pattern_index > 0;
                    ++path_index)
               {
                   const auto element_match = m_pattern[pattern_index - 1]->matches(reverse_path[path_index]);
                   if (element_match < 0)
                   {
                       return std::numeric_limits<std::size_t>::max();
                   }
                   else if (element_match == 0)
                   {
                       if (pattern_index == 0)
                       {
                           return std::numeric_limits<std::size_t>::max();
                       }
                       --pattern_index;
                   }
               }

               return pattern_index;
           }
    */
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
    /*
     namespace
    {
        const std::any& node_value()
        {
            static const std::any singleton{ 42 };
            return singleton;
        }

        const std::vector<int>& bos_preceding_edge_costs()
        {
            static const std::vector<int> singleton{};
            return singleton;
        }

        const std::vector<int>& preceding_edge_costs()
        {
            static const std::vector<int> singleton{ 1 };
            return singleton;
        }

        const std::vector<tetengo::lattice::node>& path_b_e()
        {
            static const std::vector<tetengo::lattice::node> singleton{
                tetengo::lattice::node::bos(&bos_preceding_edge_costs()),
                tetengo::lattice::node::eos(0, &preceding_edge_costs(), 0, 0)
            };
            return singleton;
        }

        const std::vector<tetengo::lattice::node>& path_b_m_s_t_e()
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

        const std::vector<tetengo::lattice::node>& path_b_m_a_t_e()
        {
            static const tetengo::lattice::string_input      key_mizuho{ "mizuho" };
            static const tetengo::lattice::string_input      key_ariake{ "ariake" };
            static const tetengo::lattice::string_input      key_tsubame{ "tsubame" };
            static const std::vector<tetengo::lattice::node> singleton{
                tetengo::lattice::node::bos(&bos_preceding_edge_costs()),
                tetengo::lattice::node{ &key_mizuho, &node_value(), 0, 0, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_ariake, &node_value(), 0, 1, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_tsubame, &node_value(), 0, 2, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node::eos(3, &preceding_edge_costs(), 0, 0)
            };
            return singleton;
        }

        const std::vector<tetengo::lattice::node>& path_b_h_t_e()
        {
            static const tetengo::lattice::string_input      key_hinokuni{ "hinokuni" };
            static const tetengo::lattice::string_input      key_tsubame{ "tsubame" };
            static const std::vector<tetengo::lattice::node> singleton{
                tetengo::lattice::node::bos(&bos_preceding_edge_costs()),
                tetengo::lattice::node{ &key_hinokuni, &node_value(), 0, 0, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_tsubame, &node_value(), 0, 2, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node::eos(3, &preceding_edge_costs(), 0, 0)
            };
            return singleton;
        }

        const std::vector<tetengo::lattice::node>& path_b_k_s_k_e()
        {
            static const tetengo::lattice::string_input      key_kamome{ "kamome" };
            static const tetengo::lattice::string_input      key_sakura{ "sakura" };
            static const tetengo::lattice::string_input      key_kumagawa{ "kumagawa" };
            static const std::vector<tetengo::lattice::node> singleton{
                tetengo::lattice::node::bos(&bos_preceding_edge_costs()),
                tetengo::lattice::node{ &key_kamome, &node_value(), 0, 0, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_sakura, &node_value(), 0, 1, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node{ &key_kumagawa, &node_value(), 0, 2, &preceding_edge_costs(), 0, 0, 0 },
                tetengo::lattice::node::eos(3, &preceding_edge_costs(), 0, 0)
            };
            return singleton;
        }

        std::vector<tetengo::lattice::node> reverse_path(const std::vector<tetengo::lattice::node>& path)
        {
            return std::vector<tetengo::lattice::node>{ std::rbegin(path), std::rend(path) };
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_e()[1]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_m_s_t_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[1]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[2]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[3]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[4]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_m_w_t_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[1]));
            pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(1));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[3]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[4]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_w_t_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(0));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[3]));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[4]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_w_s_w_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(0));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[2]));
            pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(2));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[4]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_b_w_e()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[0]));
            pattern.push_back(std::make_unique<tetengo::lattice::wildcard_constraint_element>(0));
            pattern.push_back(std::make_unique<tetengo::lattice::node_constraint_element>(path_b_m_s_t_e()[4]));
            return pattern;
        }

        std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> make_cpp_pattern_w()
        {
            std::vector<std::unique_ptr<tetengo::lattice::constraint_element>> pattern{};
            pattern.push_back(
                std::make_unique<tetengo::lattice::wildcard_constraint_element>(std::numeric_limits<std::size_t>::max()));
            return pattern;
        }

        std::vector<tetengo::lattice::node>
        make_tail(const std::vector<tetengo::lattice::node>& path, const std::size_t node_count)
        {
            assert(0 < node_count && node_count <= std::size(path));
            return std::vector<tetengo::lattice::node>{ std::next(std::begin(path), std::size(path) - node_count),
                                                        std::end(path) };
        }


    }


    BOOST_AUTO_TEST_SUITE(test_tetengo)
    BOOST_AUTO_TEST_SUITE(lattice)
    BOOST_AUTO_TEST_SUITE(constraint)


    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::constraint constraint_{};
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_e() };
        }
    }

    BOOST_AUTO_TEST_CASE(matches)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::constraint constraint_{};

            BOOST_TEST(constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_e() };

            BOOST_TEST(constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_m_s_t_e() };

            BOOST_TEST(!constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_m_w_t_e() };

            BOOST_TEST(!constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_t_e() };

            BOOST_TEST(!constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_s_w_e() };

            BOOST_TEST(!constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(!constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_b_w_e() };

            BOOST_TEST(!constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
        {
            const tetengo::lattice::constraint constraint_{ make_cpp_pattern_w() };

            BOOST_TEST(constraint_.matches(reverse_path(path_b_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_s_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_m_a_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_h_t_e())));
            BOOST_TEST(constraint_.matches(reverse_path(path_b_k_s_k_e())));
        }
    }

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
