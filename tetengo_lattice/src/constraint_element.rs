/*!
 * A constraint element.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::Debug;

use crate::node::Node;

/**
 * A constraint element.
 */
pub trait ConstraintElement: Debug {
    /**
     * Returns whether this constraint element matches the specified node.
     *
     * # Arguments
     * * `node` - A node.
     *
     * # Returns
     * * positive if this constraint element matches the specified node, and also may match its preceding nodes.
     * * 0 if this constraint element matches the specified node, and do not match its preceding nodes.
     * * negative if this constraint element does not match the specified node.
     */
    fn matches(&self, node: &Node) -> i32;
}
