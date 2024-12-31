/*!
 * A lattice.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::fmt::Debug;
use std::rc::Rc;

use anyhow::Result;

use crate::entry::EntryView;
use crate::input::Input;
use crate::node::Node;
use crate::vocabulary::Vocabulary;

/**
 * A lattice error.
 */
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum LatticeError {
    /**
     * The step is too large.
     */
    #[error("The step is too large.")]
    StepIsTooLarge,

    /**
     * No node is found for the input.
     */
    #[error("No node is found for the input.")]
    NoNodeIsFoundForTheInput,

    /**
     * No input.
     */
    #[error("No input.")]
    NoInput,
}

#[derive(Debug)]
struct GraphStep {
    input_tail: usize,
    nodes: Vec<Node>,
}

impl GraphStep {
    const fn new(input_tail: usize, nodes: Vec<Node>) -> Self {
        Self { input_tail, nodes }
    }

    const fn input_tail(&self) -> usize {
        self.input_tail
    }

    fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

/**
 * A lattice.
 */
#[derive(Debug)]
pub struct Lattice<'a> {
    vocabulary: &'a dyn Vocabulary,
    input: Option<Box<dyn Input>>,
    graph: Vec<GraphStep>,
}

impl<'a> Lattice<'a> {
    /**
     * Creates a lattice.
     *
     * # Arguments
     * * `vocabulary` - A vocabulary.
     */
    pub fn new(vocabulary: &'a dyn Vocabulary) -> Self {
        let mut self_ = Self {
            vocabulary,
            input: None,
            graph: Vec::new(),
        };
        self_.graph.push(Self::bos_step());
        self_
    }

    fn bos_step() -> GraphStep {
        let nodes = vec![Node::bos(Rc::new(Vec::new()))];
        GraphStep::new(0, nodes)
    }

    /**
     * Returns the step count.
     *
     * # Returns
     * The step count.
     */
    pub fn step_count(&self) -> usize {
        self.graph.len()
    }

    /**
     * Returns the nodes at the specified step.
     *
     * # Arguments
     * * `step` - A step.
     *
     * # Returns
     * The nodes.
     *
     * # Errors
     * * When step is too large.
     */
    pub fn nodes_at(&self, step: usize) -> Result<&[Node]> {
        if step >= self.graph.len() {
            Err(LatticeError::StepIsTooLarge.into())
        } else {
            Ok(self.graph[step].nodes.as_slice())
        }
    }

    /**
     * Pushes back an input.
     *
     * # Arguments
     * * `input` - An input.
     *
     * # Errors
     * * When no node is found for the input.
     */
    pub fn push_back(&mut self, input: Box<dyn Input>) -> Result<()> {
        if let Some(self_input) = &mut self.input {
            self_input.append(input)?;
        } else {
            self.input = Some(input);
        };
        let self_input = match &self.input {
            Some(self_input) => self_input,
            None => unreachable!(),
        };

        let mut nodes = Vec::new();
        let mut node_preceding_edge_costs = Vec::new();
        for i in 0..self.graph.len() {
            let step = &self.graph[i];

            let node_key = match self_input
                .create_subrange(step.input_tail(), self_input.length() - step.input_tail())
            {
                Ok(node_key) => node_key,
                Err(e) => return Err(e),
            };
            let found = self.vocabulary.find_entries(node_key.as_ref())?;

            let mut preceding_edge_cost_indexes = Vec::new();
            for e in &found {
                let preceding_edge_costs = self.preceding_edge_costs(step, e)?;
                preceding_edge_cost_indexes.push(node_preceding_edge_costs.len());
                node_preceding_edge_costs.push(preceding_edge_costs);
            }

            for j in 0..found.len() {
                let entry = &found[j];
                let preceding_edge_costs =
                    &node_preceding_edge_costs[preceding_edge_cost_indexes[j]];
                let best_preceding_node_index_ =
                    Self::best_preceding_node_index(step, preceding_edge_costs.as_slice());
                let best_preceding_path_cost = Self::add_cost(
                    step.nodes[best_preceding_node_index_].path_cost(),
                    preceding_edge_costs[best_preceding_node_index_],
                );
                let new_node = match Node::new_with_entry_view(
                    entry,
                    nodes.len(),
                    i,
                    preceding_edge_costs.clone(),
                    best_preceding_node_index_,
                    Self::add_cost(best_preceding_path_cost, entry.cost()),
                ) {
                    Ok(new_node) => new_node,
                    Err(e) => return Err(e),
                };
                nodes.push(new_node);
            }
        }
        if nodes.is_empty() {
            return Err(LatticeError::NoNodeIsFoundForTheInput.into());
        }

        self.graph.push(GraphStep::new(self_input.length(), nodes));

        Ok(())
    }

    /**
     * Settles this lattice.
     *
     * You can modify the lattice after settlement.
     * Modification of the lattice after settlement invalidate the EOS node.
     *
     * # Returns
     * The EOS node.
     *
     * # Errors
     * * When no input pushed yet.
     */
    pub fn settle(&mut self) -> Result<Node> {
        let Some(graph_last) = self.graph.last() else {
            return Err(LatticeError::NoInput.into());
        };
        let preceding_edge_costs = self.preceding_edge_costs(graph_last, &EntryView::BosEos)?;
        let best_preceding_node_index =
            Self::best_preceding_node_index(graph_last, preceding_edge_costs.as_slice());
        let best_preceding_path_cost = Self::add_cost(
            graph_last.nodes()[best_preceding_node_index].path_cost(),
            preceding_edge_costs[best_preceding_node_index],
        );

        let eos_node = Node::eos(
            self.graph.len() - 1,
            preceding_edge_costs,
            best_preceding_node_index,
            best_preceding_path_cost,
        );
        Ok(eos_node)
    }

    fn preceding_edge_costs(
        &self,
        step: &GraphStep,
        next_entry: &EntryView,
    ) -> Result<Rc<Vec<i32>>> {
        assert!(!step.nodes().is_empty());
        let mut costs = Vec::with_capacity(step.nodes().len());
        for node in step.nodes() {
            let cost = self.vocabulary.find_connection(node, next_entry)?.cost();
            costs.push(cost);
        }
        Ok(Rc::new(costs))
    }

    fn best_preceding_node_index(step: &GraphStep, edge_costs: &[i32]) -> usize {
        assert!(!step.nodes().is_empty());
        let mut min_index = 0;
        for i in 1..step.nodes().len() {
            if Self::add_cost(step.nodes()[i].path_cost(), edge_costs[i])
                < Self::add_cost(step.nodes()[min_index].path_cost(), edge_costs[min_index])
            {
                min_index = i;
            }
        }
        min_index
    }

    const fn add_cost(one: i32, another: i32) -> i32 {
        if one == i32::MAX || another == i32::MAX {
            i32::MAX
        } else {
            one + another
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::Entry;
    use crate::hash_map_vocabulary::HashMapVocabulary;

    use super::*;

    fn to_input(string: &str) -> Box<dyn Input> {
        Box::new(crate::string_input::StringInput::new(string.to_string()))
    }

    /*
                   +------------------mizuho/sakura/tsubame-------------------+
                   |                path cost: 4270/3220/2990                 |
                   |                                                          |
                   +------------ariake/rapid811------------+                  |
                   |          path cost: 2850/2010         |                  |
                   |                                       |                  |
        BOS--(Hakata)--kamome/local415--(Tosu)--local813--(Omuta)--local817--(Kumamoto)--EOS
                     path cost: 1640/1370   |   pc: 2830           pc: 3160   |     path cost:3390
                                            |                                 |
                                            +------------local815-------------+
                                                      path cost: 3550

        (0) 3390  BOS - tsubame - EOS
            [ sakura(3620),   local817(3760), local815(4050), mizuho(4670)   ]
        (1) 3620  BOS - sakura - EOS
            [ local817(3760), local815(4050), mizuho(4670)                   ]
        (2) 3760  BOS - rapid811 - local817 - EOS
            [ local815(4050), ariake(4600),   mizuho(4670),   local813(4680) ]
        (3) 4050  BOS - local415 - local815 - EOS
            [ kamome(4320),   ariake(4600),   mizuho(4670),   local813(4680) ]
        (4) 4320  BOS - kamome - local815 - EOS
            [ ariake(4600),   mizuho(4670),   local813(4680)                 ]
        (5) 4600  BOS - ariake - local817 - EOS
            [ mizuho(4670),   local813(4680)                                 ]
        (6) 4670  BOS - mizuho - EOS
            [ local813(4680)                                                 ]
        (7) 4680  BOS - local415 - local813 - local817 - EOS
            [ kamome(4950)                                                   ]
        (8) 4950  BOS - kamome - local813 - local817 - EOS
            [                                                                ]
        (9) ----  -
            [                                                                ]
    */
    fn entries() -> Vec<(String, Vec<Entry>)> {
        vec![
            (
                String::from("[HakataTosu][TosuOmuta][OmutaKumamoto]"),
                vec![
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta-Kumamoto")),
                        Rc::new("mizuho"),
                        3670,
                    ),
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta-Kumamoto")),
                        Rc::new("sakura"),
                        2620,
                    ),
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta-Kumamoto")),
                        Rc::new("tsubame"),
                        2390,
                    ),
                ],
            ),
            (
                String::from("[HakataTosu][TosuOmuta]"),
                vec![
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta")),
                        Rc::new("ariake"),
                        2150,
                    ),
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta")),
                        Rc::new("rapid811"),
                        1310,
                    ),
                ],
            ),
            (
                String::from("[HakataTosu]"),
                vec![
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new("kamome"), 840),
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new("local415"), 570),
                ],
            ),
            (
                String::from("[TosuOmuta]"),
                vec![Entry::new(
                    Rc::from(to_input("Tosu-Omuta")),
                    Rc::new("local813"),
                    860,
                )],
            ),
            (
                String::from("[TosuOmuta][OmutaKumamoto]"),
                vec![Entry::new(
                    Rc::from(to_input("Tosu-Omuta-Kumamoto")),
                    Rc::new("local815"),
                    1680,
                )],
            ),
            (
                String::from("[OmutaKumamoto]"),
                vec![Entry::new(
                    Rc::from(to_input("Omuta-Kumamoto")),
                    Rc::new("local817"),
                    950,
                )],
            ),
        ]
    }

    fn connections() -> Vec<((Entry, Entry), i32)> {
        vec![
            (
                (
                    Entry::BosEos,
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta-Kumamoto")),
                        Rc::new(""),
                        0,
                    ),
                ),
                600,
            ),
            (
                (
                    Entry::BosEos,
                    Entry::new(Rc::from(to_input("Hakata-Tosu-Omuta")), Rc::new(""), 0),
                ),
                700,
            ),
            (
                (
                    Entry::BosEos,
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new(""), 0),
                ),
                800,
            ),
            ((Entry::BosEos, Entry::BosEos), 8000),
            (
                (
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new(""), 0),
                    Entry::new(Rc::from(to_input("Tosu-Omuta-Kumamoto")), Rc::new(""), 0),
                ),
                500,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new(""), 0),
                    Entry::new(Rc::from(to_input("Tosu-Omuta")), Rc::new(""), 0),
                ),
                600,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Hakata-Tosu")), Rc::new(""), 0),
                    Entry::BosEos,
                ),
                6000,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Hakata-Tosu-Omuta")), Rc::new(""), 0),
                    Entry::new(Rc::from(to_input("Omuta-Kumamoto")), Rc::new(""), 0),
                ),
                200,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Hakata-Tosu-Omuta")), Rc::new(""), 0),
                    Entry::BosEos,
                ),
                2000,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Tosu-Omuta")), Rc::new(""), 0),
                    Entry::new(Rc::from(to_input("Omuta-Kumamoto")), Rc::new(""), 0),
                ),
                300,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Tosu-Omuta")), Rc::new(""), 0),
                    Entry::BosEos,
                ),
                3000,
            ),
            (
                (
                    Entry::new(
                        Rc::from(to_input("Hakata-Tosu-Omuta-Kumamoto")),
                        Rc::new(""),
                        0,
                    ),
                    Entry::BosEos,
                ),
                400,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Tosu-Omuta-Kumamoto")), Rc::new(""), 0),
                    Entry::BosEos,
                ),
                500,
            ),
            (
                (
                    Entry::new(Rc::from(to_input("Omuta-Kumamoto")), Rc::new(""), 0),
                    Entry::BosEos,
                ),
                600,
            ),
        ]
    }

    fn entry_hash(entry: &EntryView) -> u64 {
        entry.key().map_or(0, |key| key.hash_value())
    }

    fn entry_equal_to(one: &EntryView, other: &EntryView) -> bool {
        if one.key().is_none() && other.key().is_none() {
            return true;
        }
        if let Some(one_key) = one.key() {
            if let Some(other_key) = other.key() {
                return one_key.equal_to(other_key);
            }
        }
        false
    }

    fn create_vocabulary() -> Box<dyn Vocabulary> {
        Box::new(HashMapVocabulary::new(
            entries(),
            connections(),
            &entry_hash,
            &entry_equal_to,
        ))
    }

    fn create_empty_vocabulary() -> Box<dyn Vocabulary> {
        Box::new(HashMapVocabulary::new(
            Vec::new(),
            Vec::new(),
            &entry_hash,
            &entry_equal_to,
        ))
    }

    #[test]
    fn new() {
        let vocabulary = create_vocabulary();
        let _lattice = Lattice::new(vocabulary.as_ref());
    }

    #[test]
    fn step_count() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        assert_eq!(lattice.step_count(), 1);

        let result1 = lattice.push_back(to_input("[HakataTosu]"));
        assert!(result1.is_ok());
        assert_eq!(lattice.step_count(), 2);

        let result2 = lattice.push_back(to_input("[TosuOmuta]"));
        assert!(result2.is_ok());
        assert_eq!(lattice.step_count(), 3);

        let result3 = lattice.push_back(to_input("[OmutaKumamoto]"));
        assert!(result3.is_ok());
        assert_eq!(lattice.step_count(), 4);
    }

    #[test]
    fn nodes_at() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        let _result = lattice.push_back(to_input("[HakataTosu]"));
        let _result = lattice.push_back(to_input("[TosuOmuta]"));
        let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

        {
            let nodes = lattice.nodes_at(0);
            assert!(nodes.is_ok());
            let nodes = nodes.unwrap();

            assert_eq!(nodes.len(), 1);
            let preceding_edge_costs = Rc::new(Vec::new());
            assert_eq!(
                nodes[0].value().is_some(),
                Node::bos(preceding_edge_costs).value().is_some()
            );
            for (i, n) in nodes.iter().enumerate() {
                assert_eq!(n.index_in_step(), i);
            }
        }
        {
            let nodes = lattice.nodes_at(1);
            assert!(nodes.is_ok());
            let nodes = nodes.unwrap();

            assert_eq!(nodes.len(), 2);
            assert_eq!(
                nodes[0].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"kamome"
            );
            assert_eq!(
                nodes[1].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"local415"
            );
            for (i, n) in nodes.iter().enumerate() {
                assert_eq!(n.index_in_step(), i);
            }
        }
        {
            let nodes = lattice.nodes_at(2);
            assert!(nodes.is_ok());
            let nodes = nodes.unwrap();

            assert_eq!(nodes.len(), 3);
            assert_eq!(
                nodes[0].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"ariake"
            );
            assert_eq!(
                nodes[1].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"rapid811"
            );
            assert_eq!(
                nodes[2].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"local813"
            );
            for (i, n) in nodes.iter().enumerate() {
                assert_eq!(n.index_in_step(), i);
            }
        }
        {
            let nodes = lattice.nodes_at(3);
            assert!(nodes.is_ok());
            let nodes = nodes.unwrap();

            assert_eq!(nodes.len(), 5);
            assert_eq!(
                nodes[0].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"mizuho"
            );
            assert_eq!(
                nodes[1].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"sakura"
            );
            assert_eq!(
                nodes[2].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"tsubame"
            );
            assert_eq!(
                nodes[3].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"local815"
            );
            assert_eq!(
                nodes[4].value().unwrap().downcast_ref::<&str>().unwrap(),
                &"local817"
            );
            for (i, n) in nodes.iter().enumerate() {
                assert_eq!(n.index_in_step(), i);
            }
        }
        {
            let nodes = lattice.nodes_at(4);
            assert!(nodes.is_err());
        }
    }

    #[test]
    fn push_back() {
        {
            let vocabulary = create_vocabulary();
            let mut lattice = Lattice::new(vocabulary.as_ref());

            let result1 = lattice.push_back(to_input("[HakataTosu]"));
            assert!(result1.is_ok());
            let result2 = lattice.push_back(to_input("[TosuOmuta]"));
            assert!(result2.is_ok());
            let result3 = lattice.push_back(to_input("[OmutaKumamoto]"));
            assert!(result3.is_ok());
        }
        {
            let vocabulary = create_empty_vocabulary();
            let mut lattice = Lattice::new(vocabulary.as_ref());

            let result = lattice.push_back(to_input("[HakataTosu]"));
            assert!(result.is_err());
        }
    }

    #[test]
    fn settle() {
        {
            let vocabulary = create_vocabulary();
            let mut lattice = Lattice::new(vocabulary.as_ref());

            {
                let result = lattice.settle();
                let eos_node = result.unwrap();

                assert_eq!(eos_node.preceding_step(), 0);
                assert_eq!(eos_node.best_preceding_node(), 0);
                assert_eq!(eos_node.path_cost(), 8000);
            }

            let _result = lattice.push_back(to_input("[HakataTosu]"));
            {
                let result = lattice.settle();
                let eos_node = result.unwrap();

                assert_eq!(eos_node.preceding_step(), 1);
                assert_eq!(eos_node.best_preceding_node(), 1);
                assert_eq!(eos_node.path_cost(), 7370);
            }

            let _result = lattice.push_back(to_input("[TosuOmuta]"));
            {
                let result = lattice.settle();
                let eos_node = result.unwrap();

                assert_eq!(eos_node.preceding_step(), 2);
                assert_eq!(eos_node.best_preceding_node(), 1);
                assert_eq!(eos_node.path_cost(), 4010);
            }

            let _result = lattice.push_back(to_input("[OmutaKumamoto]"));
            {
                let result = lattice.settle();
                let eos_node = result.unwrap();

                assert_eq!(eos_node.preceding_step(), 3);
                assert_eq!(eos_node.best_preceding_node(), 2);
                assert_eq!(eos_node.path_cost(), 3390);
            }
        }
    }
}
