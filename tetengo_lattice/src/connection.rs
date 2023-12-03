/*!
 * A connection.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

/**
 * A connection.
 */
#[derive(Clone, Copy, Debug)]
pub struct Connection {}

impl Connection {
    // /*!
    //     \brief Creates a connection.

    //     \param cost A cost.
    // */
    // explicit constexpr connection(int cost) : m_cost{ cost } {}

    // // functions

    // /*!
    //     \brief Returns the cost.

    //     \return The cost.
    // */
    // [[nodiscard]] constexpr int cost() const
    // {
    //     return m_cost;
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // BOOST_AUTO_TEST_CASE(construction)
    // {
    //     BOOST_TEST_PASSPOINT();

    //     {
    //         [[maybe_unused]] const tetengo::lattice::connection connection_{ 42 };
    //     }

    //     {
    //         [[maybe_unused]] const tetengo_lattice_connection_t connection_{ 42 };
    //     }
    // }

    // BOOST_AUTO_TEST_CASE(cost)
    // {
    //     BOOST_TEST_PASSPOINT();

    //     {
    //         const tetengo::lattice::connection connection_{ 42 };

    //         BOOST_TEST(connection_.cost() == 42);
    //     }

    //     {
    //         const tetengo_lattice_connection_t connection_{ 42 };

    //         BOOST_TEST(connection_.cost == 42);
    //     }
    // }
}
