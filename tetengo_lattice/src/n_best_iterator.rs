/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::node::Node;

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator {
    current: i32,
    max: i32,
}

impl NBestIterator {
    /**
     * Creates an N-best lattice path iterator.
     *
     * # Arguments
     * * `max` - The maximum number of paths.
     */
    pub fn new(max: i32) -> Self {
        NBestIterator { current: 0, max }
    }
}

impl Iterator for NBestIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

struct _Cap<'a> {
    _tail_path: Vec<Node<'a>>,
    _tail_path_cost: i32,
    _whole_path_cost: i32,
}

impl<'a> _Cap<'a> {
    fn _new(tail_path: Vec<Node<'a>>, tail_path_cost: i32, whole_path_cost: i32) -> Self {
        _Cap {
            _tail_path: tail_path,
            _tail_path_cost: tail_path_cost,
            _whole_path_cost: whole_path_cost,
        }
    }
}
/*
        // functions

        /*!
            \brief Returns true if one is less than another.

            \param one One cap.
            \param another Another cap.

            \retval true  When one is less than another.
            \retval false Otherwise.
        */
        friend bool operator<(const cap& one, const cap& another);
*/
/*
    bool operator<(const cap& one, const cap& another)
    {
        return one.m_whole_path_cost < another.m_whole_path_cost;
    }
*/
/*
        /*!
            \brief Returns the tail path.

            \return The tail path.
        */
        [[nodiscard]] const std::vector<node>& tail_path() const;
*/
/*
    const std::vector<node>& cap::tail_path() const
    {
        return m_tail_path;
    }
*/
/*
        /*!
            \brief Returns the tail path cost.

            \return The tail path cost.
        */
        [[nodiscard]] int tail_path_cost() const;
*/
/*
    int cap::tail_path_cost() const
    {
        return m_tail_path_cost;
    }
*/
/*
        /*!
            \brief Returns the whole path cost.

            \return The whole path cost.
        */
        [[nodiscard]] int whole_path_cost() const;
*/
/*
    int cap::whole_path_cost() const
    {
        return m_whole_path_cost;
    }
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let iter = NBestIterator::new(3);
        let mut values = Vec::new();
        iter.for_each(|e| values.push(e));
        assert_eq!(values, vec![1, 2, 3]);
    }

    mod cap {
        use std::rc::Rc;

        use super::*;

        #[test]
        fn new() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let _cap = _Cap::_new(nodes, 24, 42);
        }
        /*
        BOOST_AUTO_TEST_CASE(operator_less)
        {
            BOOST_TEST_PASSPOINT();

            const std::vector<int>              preceding_edge_costs1{ 3, 1, 4, 1, 5, 9, 2, 6 };
            auto                                node1 = tetengo::lattice::node::eos(1, &preceding_edge_costs1, 5, 42);
            std::vector<tetengo::lattice::node> nodes1{ std::move(node1) };
            const tetengo::lattice::cap         cap1{ std::move(nodes1), 24, 42 };

            const std::vector<int>              preceding_edge_costs2{ 3, 1, 4, 1, 5, 9, 2, 6 };
            auto                                node2 = tetengo::lattice::node::eos(1, &preceding_edge_costs2, 5, 42);
            std::vector<tetengo::lattice::node> nodes2{ std::move(node2) };
            const tetengo::lattice::cap         cap2{ std::move(nodes2), 24, 42 };

            const std::vector<int>              preceding_edge_costs3{ 2, 7, 1, 8, 2, 8 };
            auto                                node3 = tetengo::lattice::node::eos(2, &preceding_edge_costs3, 3, 31);
            std::vector<tetengo::lattice::node> nodes3{ std::move(node3) };
            const tetengo::lattice::cap         cap3{ std::move(nodes3), 12, 4242 };

            BOOST_CHECK(!(cap1 < cap2));
            BOOST_CHECK(cap1 < cap3);
        }
        */
        /*
        BOOST_AUTO_TEST_CASE(tail_path)
        {
            BOOST_TEST_PASSPOINT();

            const std::vector<int>              preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            auto                                node = tetengo::lattice::node::eos(1, &preceding_edge_costs, 5, 42);
            std::vector<tetengo::lattice::node> nodes{ std::move(node) };
            const tetengo::lattice::cap         cap_{ std::move(nodes), 24, 42 };

            BOOST_TEST(std::size(cap_.tail_path()) == 1U);
            BOOST_TEST(&cap_.tail_path()[0].preceding_edge_costs() == &preceding_edge_costs);
        }
        */
        /*
        BOOST_AUTO_TEST_CASE(tail_path_cost)
        {
            BOOST_TEST_PASSPOINT();

            const std::vector<int>              preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            auto                                node = tetengo::lattice::node::eos(1, &preceding_edge_costs, 5, 42);
            std::vector<tetengo::lattice::node> nodes{ std::move(node) };
            const tetengo::lattice::cap         cap_{ std::move(nodes), 24, 42 };

            BOOST_TEST(cap_.tail_path_cost() == 24);
        }
        */
        /*
        BOOST_AUTO_TEST_CASE(whole_path_cost)
        {
            BOOST_TEST_PASSPOINT();

            const std::vector<int>              preceding_edge_costs{ 3, 1, 4, 1, 5, 9, 2, 6 };
            auto                                node = tetengo::lattice::node::eos(1, &preceding_edge_costs, 5, 42);
            std::vector<tetengo::lattice::node> nodes{ std::move(node) };
            const tetengo::lattice::cap         cap_{ std::move(nodes), 24, 42 };

            BOOST_TEST(cap_.whole_path_cost() == 42);
        }
        */
    }
}
