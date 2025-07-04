/*!
 * A node.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::entry::Entry;
use crate::error::Error;
use crate::input::Input;

/**
 * A BOS (Beginning of Sequence) node.
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bos {
    preceding_edge_costs: Rc<Vec<i32>>,
}

/**
 * A EOS (Ending of Sequence) node.
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eos {
    preceding_step: usize,
    preceding_edge_costs: Rc<Vec<i32>>,
    best_preceding_node: usize,
    path_cost: i32,
}
/**
 * A middle node.
 */
#[derive(Clone, Debug)]
pub struct Middle {
    entry: Rc<Entry>,
    index_in_step: usize,
    preceding_step: usize,
    preceding_edge_costs: Rc<Vec<i32>>,
    best_preceding_node: usize,
    path_cost: i32,
}

impl Eq for Middle {}

impl PartialEq for Middle {
    fn eq(&self, other: &Self) -> bool {
        let self_key = self
            .entry
            .key()
            .unwrap_or_else(|| unreachable!("Middle entry must have a key."));
        let other_key = other
            .entry
            .key()
            .unwrap_or_else(|| unreachable!("Middle entry must have a key."));
        self_key.equal_to(other_key)
            && self.index_in_step == other.index_in_step
            && self.preceding_step == other.preceding_step
            && self.preceding_edge_costs == other.preceding_edge_costs
            && self.best_preceding_node == other.best_preceding_node
            && self.entry.cost() == other.entry.cost()
            && self.path_cost == other.path_cost
    }
}

/**
 * A node.
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    /// The BOS (Beginning of Sequence) node.
    Bos(Bos),

    /// The EOS (Ending of Sequence) node.
    Eos(Eos),

    /// The middle node.
    Middle(Middle),
}

impl Node {
    /**
     * Creates a BOS (Beginning of Sequence).
     *
     * # Arguments
     * * `preceding_edge_costs` - Preceding edge costs.
     */
    pub const fn bos(preceding_edge_costs: Rc<Vec<i32>>) -> Self {
        Self::Bos(Bos {
            preceding_edge_costs,
        })
    }

    /**
     * Creates an EOS (Ending of Sequence).
     *
     * # Arguments
     * * `preceding_step`       - An index of a preceding step.
     * * `preceding_edge_costs` - Preceding edge costs.
     * * `best_preceding_node`  - An index of a best preceding node.
     * * `path_cost`            - A path cost.
     */
    pub const fn eos(
        preceding_step: usize,
        preceding_edge_costs: Rc<Vec<i32>>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Self {
        Self::Eos(Eos {
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            path_cost,
        })
    }

    /**
     * Creates a node.
     *
     * # Arguments
     * * `key`                  - A key.
     * * `value`                - A value.
     * * `index_in_step`        - An index in the step.
     * * `preceding_step`       - An index of a preceding step.
     * * `preceding_edge_costs` - Preceding edge costs.
     * * `best_preceding_node`  - An index of a best preceding node.
     * * `node_cost`            - A node cost.
     * * `path_cost`            - A path cost.
     */
    pub fn new(
        key: Box<dyn Input>,
        value: Box<dyn Any>,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: Rc<Vec<i32>>,
        best_preceding_node: usize,
        node_cost: i32,
        path_cost: i32,
    ) -> Self {
        let entry = Rc::new(Entry::new(key, value, node_cost));
        Self::Middle(Middle {
            entry,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            path_cost,
        })
    }

    /**
     * Creates a node with a vocabulary entry.
     *
     * # Errors
     * * When `entry` is BOS or EOS.
     */
    pub fn new_with_entry(
        entry: Rc<Entry>,
        index_in_step: usize,
        preceding_step: usize,
        preceding_edge_costs: Rc<Vec<i32>>,
        best_preceding_node: usize,
        path_cost: i32,
    ) -> Result<Self, Error> {
        if entry.is_bos_eos() {
            return Err(Error::BosOrEosEntryNotAllowed);
        }
        Ok(Self::Middle(Middle {
            entry,
            index_in_step,
            preceding_step,
            preceding_edge_costs,
            best_preceding_node,
            path_cost,
        }))
    }

    pub(crate) fn entry(&self) -> Rc<Entry> {
        match self {
            Self::Bos(_) | Self::Eos(_) => Rc::new(Entry::BosEos),
            Self::Middle(middle) => middle.entry.clone(),
        }
    }

    /**
     * Returns the key.
     *
     * # Returns
     * The key.
     */
    pub fn key(&self) -> Option<&dyn Input> {
        match self {
            Self::Bos(_) | Self::Eos(_) => Entry::BosEos.key(),
            Self::Middle(middle) => middle.entry.key(),
        }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub fn value(&self) -> Option<&dyn Any> {
        match self {
            Self::Bos(_) | Self::Eos(_) => Entry::BosEos.value(),
            Self::Middle(middle) => middle.entry.value(),
        }
    }

    /**
     * Returns the index in the step.
     *
     * # Returns
     * The index in the step.
     */
    pub const fn index_in_step(&self) -> usize {
        match self {
            Self::Bos(_) | Self::Eos(_) => 0,
            Self::Middle(middle) => middle.index_in_step,
        }
    }

    /**
     * Returns the preceding step.
     *
     * # Returns
     * The preceding step.
     */
    pub const fn preceding_step(&self) -> usize {
        match self {
            Self::Bos(_) => usize::MAX,
            Self::Eos(eos) => eos.preceding_step,
            Self::Middle(middle) => middle.preceding_step,
        }
    }

    /**
     * Returns the preceding edge costs.
     *
     * # Returns
     * The preceding edge costs.
     */
    pub fn preceding_edge_costs(&self) -> &Vec<i32> {
        match self {
            Self::Bos(bos) => bos.preceding_edge_costs.as_ref(),
            Self::Eos(eos) => eos.preceding_edge_costs.as_ref(),
            Self::Middle(middle) => middle.preceding_edge_costs.as_ref(),
        }
    }

    /**
     * Returns the index of the best preceding node.
     *
     * # Returns
     * The index of the best preceding node.
     */
    pub const fn best_preceding_node(&self) -> usize {
        match self {
            Self::Bos(_) => usize::MAX,
            Self::Eos(eos) => eos.best_preceding_node,
            Self::Middle(middle) => middle.best_preceding_node,
        }
    }

    /**
     * Returns the node cost.
     *
     * # Returns
     * The node cost.
     */
    pub fn node_cost(&self) -> i32 {
        match self {
            Self::Bos(_) | Self::Eos(_) => Entry::BosEos.cost(),
            Self::Middle(middle) => middle.entry.cost(),
        }
    }

    /**
     * Returns the path cost.
     *
     * # Returns
     * The path cost.
     */
    pub const fn path_cost(&self) -> i32 {
        match self {
            Self::Bos(_) => 0,
            Self::Eos(eos) => eos.path_cost,
            Self::Middle(middle) => middle.path_cost,
        }
    }

    /**
     * Returns `true` if this node is the BOS.
     *
     * # Returns
     * `true` if this node is the BOS.
     */
    pub const fn is_bos(&self) -> bool {
        matches!(self, Self::Bos(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::string_input::StringInput;

    use super::*;

    #[test]
    fn bos() {
        let preceding_edge_costs = Rc::new(Vec::new());
        let bos = Node::bos(preceding_edge_costs.clone());

        assert!(bos.key().is_none());
        assert!(bos.value().is_none());
        assert_eq!(bos.index_in_step(), 0);
        assert_eq!(bos.preceding_step(), usize::MAX);
        assert_eq!(bos.preceding_edge_costs(), preceding_edge_costs.as_ref());
        assert_eq!(bos.best_preceding_node(), usize::MAX);
        assert_eq!(bos.node_cost(), Entry::BosEos.cost());
        assert_eq!(bos.path_cost(), 0);
    }

    #[test]
    fn eos() {
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let eos = Node::eos(1, preceding_edge_costs.clone(), 5, 42);

        assert!(eos.key().is_none());
        assert!(eos.value().is_none());
        assert_eq!(eos.index_in_step(), 0);
        assert_eq!(eos.preceding_step(), 1);
        assert_eq!(eos.preceding_edge_costs(), preceding_edge_costs.as_ref());
        assert_eq!(eos.best_preceding_node(), 5);
        assert_eq!(eos.node_cost(), Entry::BosEos.cost());
        assert_eq!(eos.path_cost(), 42);
    }

    #[test]
    fn new() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let _node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );
    }

    #[test]
    fn new_with_entry() {
        {
            let entry_key = StringInput::new(String::from("mizuho"));
            let entry_value = 42;
            let entry = Rc::new(Entry::new(
                Box::new(entry_key.clone()),
                Box::new(entry_value),
                24,
            ));
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::new_with_entry(entry, 53, 1, preceding_edge_costs.clone(), 5, 2424);

            let node = node.unwrap();
            assert_eq!(
                node.key().unwrap().downcast_ref::<StringInput>().unwrap(),
                &entry_key
            );
            assert_eq!(node.value().unwrap().downcast_ref::<i32>().unwrap(), &42);
            assert_eq!(node.index_in_step(), 53);
            assert_eq!(node.preceding_step(), 1);
            assert_eq!(node.preceding_edge_costs(), preceding_edge_costs.as_ref());
            assert_eq!(node.best_preceding_node(), 5);
            assert_eq!(node.node_cost(), 24);
            assert_eq!(node.path_cost(), 2424);
        }
        {
            let entry = Rc::new(Entry::BosEos);
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::new_with_entry(entry, 53, 1, preceding_edge_costs, 5, 2424);

            assert!(node.is_err());
        }
    }

    #[test]
    fn key() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(
            node.key()
                .unwrap()
                .downcast_ref::<StringInput>()
                .unwrap()
                .value(),
            "mizuho"
        );
    }

    #[test]
    fn value() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.value().unwrap().downcast_ref::<i32>().unwrap(), &42);
    }

    #[test]
    fn index_in_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.index_in_step(), 53);
    }

    #[test]
    fn preceding_step() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.preceding_step(), 1);
    }

    #[test]
    fn preceding_edge_costs() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs.clone(),
            5,
            24,
            2424,
        );

        assert_eq!(node.preceding_edge_costs(), preceding_edge_costs.as_ref());
    }

    #[test]
    fn best_preceding_node() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.best_preceding_node(), 5);
    }

    #[test]
    fn node_cost() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.node_cost(), 24);
    }

    #[test]
    fn path_cost() {
        let key = StringInput::new(String::from("mizuho"));
        let value = 42;
        let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node = Node::new(
            Box::new(key),
            Box::new(value),
            53,
            1,
            preceding_edge_costs,
            5,
            24,
            2424,
        );

        assert_eq!(node.path_cost(), 2424);
    }

    #[test]
    fn is_bos() {
        {
            let preceding_edge_costs_bos = Rc::new(Vec::new());
            assert!(Node::bos(preceding_edge_costs_bos).is_bos());
        }
        {
            let preceding_edge_costs_eos = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            assert!(!Node::eos(1, preceding_edge_costs_eos, 5, 42).is_bos());
        }
        {
            let key = StringInput::new(String::from("mizuho"));
            let value = 42;
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            assert!(
                !Node::new(
                    Box::new(key),
                    Box::new(value),
                    53,
                    1,
                    preceding_edge_costs,
                    5,
                    24,
                    2424
                )
                .is_bos()
            );
        }
    }

    #[test]
    fn eq() {
        let key = StringInput::new(String::from("mizuho"));

        let preceding_edge_costs_bos = Rc::new(Vec::new());
        let bos = Node::bos(preceding_edge_costs_bos);

        let preceding_edge_costs_eos = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let eos = Node::eos(1, preceding_edge_costs_eos, 5, 42);

        let value1 = 42;
        let preceding_edge_costs1 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node1 = Node::new(
            Box::new(key.clone()),
            Box::new(value1),
            53,
            1,
            preceding_edge_costs1,
            5,
            24,
            2424,
        );

        let value2 = 42;
        let preceding_edge_costs2 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let node2 = Node::new(
            Box::new(key),
            Box::new(value2),
            53,
            1,
            preceding_edge_costs2,
            5,
            24,
            2424,
        );

        assert_eq!(bos, bos);
        assert_ne!(bos, eos);
        assert_ne!(bos, node1);
        assert_eq!(node1, node2);
    }
}
