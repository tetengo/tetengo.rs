/*!
 * A connection.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

/**
 * A connection.
 */
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    cost: i32,
}

impl Connection {
    /**
     * Creates a connection.
     *
     * # Arguments
     * * `cost` - A cost.
     */
    pub const fn new(cost: i32) -> Self {
        Self { cost }
    }

    /**
     * Returns the cost.
     *
     * # Returns
     * The cost.
     */
    pub const fn cost(&self) -> i32 {
        self.cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = Connection::new(42);
    }

    #[test]
    fn cost() {
        let connection_ = Connection::new(42);

        assert_eq!(connection_.cost(), 42);
    }
}
