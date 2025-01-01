/*!
 * A path.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use crate::node::Node;

/**
 * A path.
 */
#[derive(Debug, Default)]
pub struct Path {
    nodes: Vec<Node>,
    cost: i32,
}

impl Path {
    /**
     * Creates a path.
     *
     * # Arguments
     * * `nodes` - Nodes.
     * * `cost`  - A cost.
     */
    pub const fn new(nodes: Vec<Node>, cost: i32) -> Self {
        Path { nodes, cost }
    }

    /**
     * Returns `true` if this path is empty.
     *
     * # Returns
     * `true` if this path is empty.
     */
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /**
     * Returns the nodes.
     *
     * # Returns
     * The nodes.
     */
    pub fn nodes(&self) -> &[Node] {
        self.nodes.as_slice()
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
    use std::rc::Rc;
    use std::sync::LazyLock;

    use crate::string_input::StringInput;

    use super::*;

    const NODE_VALUE: i32 = 42;

    const BOS_PRECEDING_EDGE_COSTS: Vec<i32> = vec![];

    static PRECEDING_EDGE_COSTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1]);

    fn make_nodes() -> Vec<Node> {
        let key_mizuho = Rc::new(StringInput::new(String::from("mizuho")));
        let key_sakura = Rc::new(StringInput::new(String::from("sakura")));
        let key_tsubame = Rc::new(StringInput::new(String::from("tsubame")));
        vec![
            Node::bos(Rc::new(BOS_PRECEDING_EDGE_COSTS)),
            Node::new(
                key_mizuho,
                Rc::new(NODE_VALUE),
                0,
                0,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::new(
                key_sakura,
                Rc::new(NODE_VALUE),
                0,
                1,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::new(
                key_tsubame,
                Rc::new(NODE_VALUE),
                0,
                2,
                Rc::new(PRECEDING_EDGE_COSTS.clone()),
                0,
                0,
                0,
            ),
            Node::eos(3, Rc::new(PRECEDING_EDGE_COSTS.clone()), 0, 0),
        ]
    }

    #[test]
    fn new() {
        let _path = Path::new(make_nodes(), 42);
    }

    #[test]
    fn is_empty() {
        {
            let path = Path::new(Vec::new(), 0);
            assert!(path.is_empty());
        }
        {
            let path = Path::new(make_nodes(), 42);
            assert!(!path.is_empty());
        }
    }

    #[test]
    fn nodes() {
        {
            let path = Path::new(Vec::new(), 0);
            assert!(path.nodes().is_empty());
        }
        {
            let path = Path::new(make_nodes(), 42);
            assert_eq!(path.nodes(), make_nodes().as_slice());
        }
    }

    #[test]
    fn cost() {
        let path = Path::new(make_nodes(), 42);
        assert_eq!(path.cost(), 42);
    }
}
