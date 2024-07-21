/*!
 * A vocabulary.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::Debug;

use crate::connection::Connection;
use crate::entry::EntryView;
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
     * Entry views.
     */
    fn find_entries(&self, key: &dyn Input) -> Vec<EntryView<'_>>;

    /**
     * Finds a connection between an origin node and a destination entry.
     *
     * # Arguments
     * * `from` - An origin node.
     * * `to`   - A destination entry.
     *
     * # Returns
     * A connection between the origin node and the destination entry.
     */
    fn find_connection(&self, from: &Node<'_>, to: &EntryView<'_>) -> Connection;
}
