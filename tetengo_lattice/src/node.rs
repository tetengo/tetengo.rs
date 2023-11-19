/*!
 * A node.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::{self, Debug, Formatter};

use crate::input::Input;

/**
 * A middle node.
 */
#[derive(Clone, Copy)]
pub struct Middle<'a> {
    _input: Option<&'a dyn Input>,
}

impl Debug for Middle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddleEntryView")
            .field("input", &"Option<&dyn Input>")
            .finish()
    }
}

/**
 * A node.
 */
#[derive(Debug, Clone)]
pub enum Node<'a> {
    /// The BOS (Beginning of Sequence) entry.
    Bos,

    /// The EOS (Ending of Sequence) entry.
    Eos,

    /// The middle entry.
    Middle(Middle<'a>),
}

impl Node<'_> {
    /*
        /*!
            \brief Returns the BOS (Beginning of Sequence).

            \param p_preceding_edge_costs A pointer to preceding edge costs.

            \return The BOS.

            \throw std::invalid_argument When p_preceding_edge_costs is nullptr.
        */
        [[nodiscard]] static node bos(const std::vector<int>* p_preceding_edge_costs);
    */
    /*
        /*!
            \brief Returns an EOS (End of Sequence).

            \param preceding_step         An index of a preceding step.
            \param p_preceding_edge_costs A pointer to preceding edge costs.
            \param best_preceding_node    An index of a best preceding node.
            \param path_cost              A path cost.

            \return An EOS.

            \throw std::invalid_argument When p_preceding_edge_costs is nullptr.
        */
        [[nodiscard]] static node
        eos(std::size_t             preceding_step,
            const std::vector<int>* p_preceding_edge_costs,
            std::size_t             best_preceding_node,
            int                     path_cost);
    */
    /*
        /*!
            \brief Creates a node.

            \param p_key                  A pointer to a key.
            \param p_value                A pointer to a value.
            \param index_in_step          An index in the step.
            \param preceding_step         An index of a preceding step.
            \param p_preceding_edge_costs A pointer to preceding edge costs.
            \param best_preceding_node    An index of a best preceding node.
            \param node_cost              A node cost.
            \param path_cost              A path cost.
        */
        constexpr node(
            const input*            p_key,
            const std::any*         p_value,
            std::size_t             index_in_step,
            std::size_t             preceding_step,
            const std::vector<int>* p_preceding_edge_costs,
            std::size_t             best_preceding_node,
            int                     node_cost,
            int                     path_cost) :
        m_p_key{ p_key },
        m_p_value{ p_value },
        m_index_in_step{ index_in_step },
        m_preceding_step{ preceding_step },
        m_p_preceding_edge_costs{ p_preceding_edge_costs },
        m_best_preceding_node{ best_preceding_node },
        m_node_cost{ node_cost },
        m_path_cost{ path_cost }
        {}
    */
    /*
        /*!
            \brief Creates a node from a vocabulary entry.

            \param entry                  An entry.
            \param index_in_step          An index in the step.
            \param preceding_step         An index of a preceding step.
            \param p_preceding_edge_costs A pointer to preceding edge costs.
            \param best_preceding_node    An index of a best preceding node.
            \param path_cost              A path cost.

            \throw std::invalid_argument When p_preceding_edge_costs is nullptr.
        */
        constexpr node(
            const entry_view&       entry,
            std::size_t             index_in_step,
            std::size_t             preceding_step,
            const std::vector<int>* p_preceding_edge_costs,
            std::size_t             best_preceding_node,
            int                     path_cost) :
        node{ entry.p_key(),          entry.value(),       index_in_step, preceding_step,
                p_preceding_edge_costs, best_preceding_node, entry.cost(),  path_cost }
        {}
    */
    /*
        /*!
            \brief Returns true if one node is equal to another.

            \param one     One node.
            \param another Another node.

            \retval true  When one node is equal to another.
            \retval valse Otherwise.
        */
        friend constexpr bool operator==(const node& one, const node& another)
        {
            return ((!one.p_key() && !another.p_key()) ||
                    (one.p_key() && another.p_key() && *one.p_key() == *another.p_key())) &&
                    one.preceding_step() == another.preceding_step() &&
                    one.best_preceding_node() == another.best_preceding_node() &&
                    one.node_cost() == another.node_cost() && one.path_cost() == another.path_cost();
        }
    */
    /*
        /*!
            \brief Returns the key.

            \return The key.
        */
        [[nodiscard]] constexpr const input* p_key() const
        {
            return m_p_key;
        }
    */
    /*
        /*!
            \brief Returns the value.

            \return The value.
        */
        [[nodiscard]] constexpr const std::any& value() const
        {
            assert(m_p_value);
            return *m_p_value;
        }
    */
    /*
        /*!
            \brief Returns the index in the step.

            \return The index in the step.
        */
        [[nodiscard]] constexpr std::size_t index_in_step() const
        {
            return m_index_in_step;
        }
        /*!
            \brief Returns the index of the preceding step.

            \return The index of the preceding step.
        */
        [[nodiscard]] constexpr std::size_t preceding_step() const
        {
            return m_preceding_step;
        }
    */
    /*
        /*!
            \brief Returns the preceding edge costs.

            \return The preceding edge costs.
        */
        [[nodiscard]] constexpr const std::vector<int>& preceding_edge_costs() const
        {
            assert(m_p_preceding_edge_costs);
            return *m_p_preceding_edge_costs;
        }
    */
    /*
        /*!
            \brief Returns the index of the best preceding node.

            \return The index of the best preceding node.
        */
        [[nodiscard]] constexpr std::size_t best_preceding_node() const
        {
            return m_best_preceding_node;
        }
    */
    /*
        /*!
            \brief Returns the node cost.

            \return The node cost.
        */
        [[nodiscard]] constexpr int node_cost() const
        {
            return m_node_cost;
        }
    */
    /*
        /*!
            \brief Returns the path cost.

            \return The path cost.
        */
        [[nodiscard]] constexpr int path_cost() const
        {
            return m_path_cost;
        }
    */
    /*
        /*!
            \brief Returns true is this node is the BOS.

            \retval true  When this node is the BOS.
            \retval false Otherwise.
        */
        [[nodiscard]] bool is_bos() const;
    */
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {
        // assert!(true);
    }
}
