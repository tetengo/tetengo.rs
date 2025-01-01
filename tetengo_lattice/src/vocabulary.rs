/*!
 * A vocabulary.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::Debug;

use anyhow::Result;

use crate::connection::Connection;
use crate::entry::Entry;
use crate::input::Input;
use crate::node::Node;

/**
 * A vocabulary.
 */
pub trait Vocabulary: Debug {
    /**
     * Finds entries.
     *
     * # Arguments
     * * `key` - A key.
     *
     * # Returns
     * Entries.
     *
     * # Errors
     * * When finding entries fails.
     */
    fn find_entries(&self, key: &dyn Input) -> Result<Vec<Entry>>;

    /**
     * Finds a connection between an origin node and a destination entry.
     *
     * # Arguments
     * * `from` - An origin node.
     * * `to`   - A destination entry.
     *
     * # Returns
     * A connection between the origin node and the destination entry.
     *
     * # Errors
     * * When finding a connection fails.
     */
    fn find_connection(&self, from: &Node, to: &Entry) -> Result<Connection>;
}
