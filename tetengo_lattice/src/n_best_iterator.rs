/*!
 * An N-best lattice path iterator.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use crate::constraint::Constraint;
use crate::lattice::Lattice;
use crate::node::Node;
use crate::path::Path;

/**
 * An N-best lattice path iterator.
 */
#[derive(Debug)]
pub struct NBestIterator<'a> {
    lattice: &'a Lattice<'a>,
    caps: BinaryHeap<Reverse<Cap>>,
    constraint: Box<Constraint<'a>>,
}

impl<'a> NBestIterator<'a> {
    /**
     * Creates an iterator.
     *
     * # Arguments
     * * `lattice`    - A lattice.
     * * `eos_node`   - An EOS node.
     * * `constraint` - A constraint.
     */
    pub fn new(lattice: &'a Lattice<'a>, eos_node: Node, constraint: Box<Constraint<'a>>) -> Self {
        let mut caps = BinaryHeap::new();
        let tail_path_cost = eos_node.node_cost();
        let whole_path_cost = eos_node.path_cost();
        caps.push(Reverse(Cap::new(
            vec![eos_node],
            tail_path_cost,
            whole_path_cost,
        )));
        Self {
            lattice,
            caps,
            constraint,
        }
    }

    fn open_cap(
        lattice: &Lattice<'a>,
        caps: &mut BinaryHeap<Reverse<Cap>>,
        constraint: &Constraint<'a>,
    ) -> Option<Path> {
        let mut path = None;
        while !caps.is_empty() {
            let Some(opened) = caps.pop() else {
                unreachable!("caps must not be empty.");
            };
            let opened = opened.0;

            let mut next_path = opened.tail_path().to_vec();
            let mut tail_path_cost = opened.tail_path_cost();
            let mut nonconforming_path = false;
            let Some(mut node) = opened.tail_path().last() else {
                unreachable!("tail_path must not be empty.");
            };
            while !node.is_bos() {
                let Ok(preceding_nodes) = lattice.nodes_at(node.preceding_step()) else {
                    unreachable!("preceding_step must be within the preceding steps in lattice.");
                };
                for (i, preceding_node) in preceding_nodes.iter().enumerate() {
                    if i == node.best_preceding_node() {
                        continue;
                    }
                    let mut cap_tail_path = next_path.clone();
                    cap_tail_path.push(preceding_node.clone());
                    if !constraint.matches_tail(&cap_tail_path) {
                        continue;
                    }
                    let preceding_edge_cost = node.preceding_edge_costs()[i];
                    let cap_tail_path_cost = Self::add_cost(
                        Self::add_cost(tail_path_cost, preceding_edge_cost),
                        preceding_node.node_cost(),
                    );
                    if cap_tail_path_cost == i32::MAX {
                        continue;
                    }
                    let cap_whole_path_cost = Self::add_cost(
                        Self::add_cost(tail_path_cost, preceding_edge_cost),
                        preceding_node.path_cost(),
                    );
                    if cap_whole_path_cost == i32::MAX {
                        continue;
                    }
                    caps.push(Reverse(Cap::new(
                        cap_tail_path,
                        cap_tail_path_cost,
                        cap_whole_path_cost,
                    )));
                }

                let best_preceding_edge_cost =
                    node.preceding_edge_costs()[node.best_preceding_node()];
                let best_preceding_node = &preceding_nodes[node.best_preceding_node()];
                next_path.push(best_preceding_node.clone());
                if !constraint.matches_tail(&next_path) {
                    nonconforming_path = true;
                    break;
                }
                tail_path_cost = Self::add_cost(
                    tail_path_cost,
                    Self::add_cost(best_preceding_edge_cost, best_preceding_node.node_cost()),
                );

                node = best_preceding_node;
            }

            if !nonconforming_path {
                assert!(constraint.matches(&next_path));
                let reversed_next_path = next_path.iter().rev().cloned().collect();
                path = Some(Path::new(reversed_next_path, opened.whole_path_cost()));
                break;
            }
        }

        path
    }

    const fn add_cost(one: i32, another: i32) -> i32 {
        if one == i32::MAX || another == i32::MAX {
            i32::MAX
        } else {
            one + another
        }
    }
}

impl Iterator for NBestIterator<'_> {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        if self.caps.is_empty() {
            None
        } else {
            Self::open_cap(self.lattice, &mut self.caps, self.constraint.as_ref())
        }
    }
}

#[derive(Debug, Eq)]
struct Cap {
    tail_path: Vec<Node>,
    tail_path_cost: i32,
    whole_path_cost: i32,
}

impl Cap {
    const fn new(tail_path: Vec<Node>, tail_path_cost: i32, whole_path_cost: i32) -> Self {
        Cap {
            tail_path,
            tail_path_cost,
            whole_path_cost,
        }
    }

    fn tail_path(&self) -> &[Node] {
        self.tail_path.as_slice()
    }

    const fn tail_path_cost(&self) -> i32 {
        self.tail_path_cost
    }

    const fn whole_path_cost(&self) -> i32 {
        self.whole_path_cost
    }
}

impl Ord for Cap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.whole_path_cost.cmp(&other.whole_path_cost)
    }
}

impl PartialEq for Cap {
    fn eq(&self, other: &Self) -> bool {
        self.whole_path_cost == other.whole_path_cost
    }
}

impl PartialOrd for Cap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.whole_path_cost.cmp(&other.whole_path_cost))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::constraint_element::ConstraintElement;
    use crate::entry::{Entry, EntryView};
    use crate::hash_map_vocabulary::HashMapVocabulary;
    use crate::input::Input;
    use crate::node_constraint_element::NodeConstraintElement;
    use crate::string_input::StringInput;
    use crate::vocabulary::Vocabulary;
    use crate::wildcard_constraint_element::WildcardConstraintElement;

    use super::*;

    fn to_input(string: &str) -> Box<dyn Input> {
        Box::new(StringInput::new(string.to_string()))
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

    fn preceding_edge_cost(path: &Path, node_index: usize) -> i32 {
        let nodes = path.nodes();
        assert!(!nodes.is_empty());
        assert!(0 < node_index && node_index < nodes.len());
        nodes[node_index].preceding_edge_costs()[nodes[node_index - 1].index_in_step()]
    }

    fn recalc_path_cost(path: &Path) -> i32 {
        let nodes = path.nodes();
        assert!(!nodes.is_empty());
        let mut cost = nodes[0].node_cost();
        for (i, node) in nodes.iter().enumerate().skip(1) {
            cost += preceding_edge_cost(path, i);
            cost += node.node_cost();
        }
        cost
    }

    #[test]
    fn new() {
        let vocabulary = create_vocabulary();
        let mut lattice = Lattice::new(vocabulary.as_ref());
        let _result = lattice.push_back(to_input("[HakataTosu]"));
        let _result = lattice.push_back(to_input("[TosuOmuta]"));
        let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

        let eos_node = lattice.settle().unwrap();
        let _iterator = NBestIterator::new(&lattice, eos_node, Box::new(Constraint::new()));
    }

    #[test]
    fn next() {
        {
            let vocabulary = create_vocabulary();
            let mut lattice = Lattice::new(vocabulary.as_ref());
            let _result = lattice.push_back(to_input("[HakataTosu]"));
            let _result = lattice.push_back(to_input("[TosuOmuta]"));
            let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

            let eos_node = lattice.settle().unwrap();
            let mut iterator = NBestIterator::new(&lattice, eos_node, Box::new(Constraint::new()));

            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 3);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"tsubame"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 600);
                assert!(path.nodes()[2].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 2), 400);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 3);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"sakura"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 600);
                assert!(path.nodes()[2].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 2), 400);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"rapid811"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 700);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local817"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 200);
                assert!(path.nodes()[3].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 3), 600);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local415"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 800);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local815"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 500);
                assert!(path.nodes()[3].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 3), 500);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"kamome"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 800);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local815"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 500);
                assert!(path.nodes()[3].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 3), 500);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"ariake"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 700);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local817"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 200);
                assert!(path.nodes()[3].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 3), 600);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 3);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"mizuho"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 600);
                assert!(path.nodes()[2].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 2), 400);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 5);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local415"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 800);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local813"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 600);
                assert_eq!(
                    path.nodes()[3]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local817"
                );
                assert_eq!(preceding_edge_cost(&path, 3), 300);
                assert!(path.nodes()[4].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 4), 600);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 5);
                assert!(path.nodes()[0].value().is_none());
                assert_eq!(
                    path.nodes()[1]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"kamome"
                );
                assert_eq!(preceding_edge_cost(&path, 1), 800);
                assert_eq!(
                    path.nodes()[2]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local813"
                );
                assert_eq!(preceding_edge_cost(&path, 2), 600);
                assert_eq!(
                    path.nodes()[3]
                        .value()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<&str>()
                        .unwrap(),
                    &"local817"
                );
                assert_eq!(preceding_edge_cost(&path, 3), 300);
                assert!(path.nodes()[4].value().is_none());
                assert_eq!(preceding_edge_cost(&path, 4), 600);
                assert_eq!(recalc_path_cost(&path), path.cost());
            }
            assert!(iterator.next().is_none());
        }
        {
            let vocabulary = create_vocabulary();
            let mut lattice = Lattice::new(vocabulary.as_ref());
            let _result = lattice.push_back(to_input("[HakataTosu]"));
            let _result = lattice.push_back(to_input("[TosuOmuta]"));
            let _result = lattice.push_back(to_input("[OmutaKumamoto]"));

            let eos_node = lattice.settle().unwrap();
            let mut iterator =
                NBestIterator::new(&lattice, eos_node.clone(), Box::new(Constraint::new()));

            {
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 3);

                let pattern: Vec<Box<dyn ConstraintElement>> = vec![
                    Box::new(NodeConstraintElement::new(path.nodes()[0].clone())),
                    Box::new(NodeConstraintElement::new(path.nodes()[1].clone())),
                    Box::new(NodeConstraintElement::new(path.nodes()[2].clone())),
                ];
                let constraint = Box::new(Constraint::new_with_pattern(pattern));

                let mut constrained_iterator =
                    NBestIterator::new(&lattice, eos_node.clone(), constraint);

                let constrained_path = constrained_iterator.next().unwrap();
                assert_eq!(constrained_path.nodes(), path.nodes());
            }
            {
                let _skipped = iterator.next().unwrap();
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);

                let pattern: Vec<Box<dyn ConstraintElement>> = vec![
                    Box::new(NodeConstraintElement::new(path.nodes()[0].clone())),
                    Box::new(NodeConstraintElement::new(path.nodes()[1].clone())),
                    Box::new(NodeConstraintElement::new(path.nodes()[2].clone())),
                    Box::new(NodeConstraintElement::new(path.nodes()[3].clone())),
                ];
                let constraint = Box::new(Constraint::new_with_pattern(pattern));

                let mut constrained_iterator =
                    NBestIterator::new(&lattice, eos_node.clone(), constraint);

                let constrained_path = constrained_iterator.next().unwrap();
                assert_eq!(constrained_path.nodes(), path.nodes());
            }
            {
                let _skipped = iterator.next().unwrap();
                let path = iterator.next().unwrap();
                assert_eq!(path.nodes().len(), 4);

                {
                    let pattern: Vec<Box<dyn ConstraintElement>> = vec![
                        Box::new(NodeConstraintElement::new(path.nodes()[0].clone())),
                        Box::new(NodeConstraintElement::new(path.nodes()[1].clone())),
                        Box::new(WildcardConstraintElement::new(1)),
                        Box::new(NodeConstraintElement::new(path.nodes()[3].clone())),
                    ];
                    let constraint = Box::new(Constraint::new_with_pattern(pattern));

                    let mut constrained_iterator =
                        NBestIterator::new(&lattice, eos_node.clone(), constraint);

                    {
                        let constrained_path = constrained_iterator.next().unwrap();
                        assert_eq!(constrained_path.nodes(), path.nodes());
                    }
                    {
                        let constrained_path = constrained_iterator.next().unwrap();
                        assert_eq!(constrained_path.nodes().len(), 5);
                        assert_eq!(constrained_path.nodes()[0], path.nodes()[0]);
                        assert_eq!(constrained_path.nodes()[1], path.nodes()[1]);
                        assert!(constrained_path.nodes()[2].key().is_some());
                        assert!(constrained_path.nodes()[2]
                            .key()
                            .unwrap()
                            .as_any()
                            .downcast_ref::<StringInput>()
                            .is_some());
                        assert_eq!(
                            constrained_path.nodes()[2]
                                .key()
                                .unwrap()
                                .as_any()
                                .downcast_ref::<StringInput>()
                                .unwrap()
                                .value(),
                            "Tosu-Omuta"
                        );
                        assert!(constrained_path.nodes()[3].key().is_some());
                        assert!(constrained_path.nodes()[3]
                            .key()
                            .unwrap()
                            .as_any()
                            .downcast_ref::<StringInput>()
                            .is_some());
                        assert_eq!(
                            constrained_path.nodes()[3]
                                .key()
                                .unwrap()
                                .as_any()
                                .downcast_ref::<StringInput>()
                                .unwrap()
                                .value(),
                            "Omuta-Kumamoto"
                        );
                        assert_eq!(constrained_path.nodes()[4], path.nodes()[3]);
                    }
                    assert!(constrained_iterator.next().is_none());
                }
                {
                    let pattern: Vec<Box<dyn ConstraintElement>> = vec![
                        Box::new(NodeConstraintElement::new(path.nodes()[0].clone())),
                        Box::new(WildcardConstraintElement::new(0)),
                        Box::new(NodeConstraintElement::new(path.nodes()[2].clone())),
                        Box::new(NodeConstraintElement::new(path.nodes()[3].clone())),
                    ];
                    let constraint = Box::new(Constraint::new_with_pattern(pattern));

                    let mut constrained_iterator =
                        NBestIterator::new(&lattice, eos_node.clone(), constraint);

                    {
                        let constraint_path = constrained_iterator.next().unwrap();
                        assert_eq!(constraint_path.nodes().len(), 4);
                        assert_eq!(constraint_path.nodes()[0], path.nodes()[0]);
                        assert_eq!(
                            constraint_path.nodes()[1]
                                .value()
                                .unwrap()
                                .as_any()
                                .downcast_ref::<&str>()
                                .unwrap(),
                            &"local415"
                        );
                        assert_eq!(constraint_path.nodes()[2], path.nodes()[2]);
                        assert_eq!(constraint_path.nodes()[3], path.nodes()[3]);
                    }
                    {
                        let constraint_path = constrained_iterator.next().unwrap();
                        assert_eq!(constraint_path.nodes(), path.nodes());
                    }
                    assert!(constrained_iterator.next().is_none());
                }
                {
                    let pattern: Vec<Box<dyn ConstraintElement>> = vec![
                        Box::new(NodeConstraintElement::new(path.nodes()[0].clone())),
                        Box::new(WildcardConstraintElement::new(0)),
                        Box::new(NodeConstraintElement::new(path.nodes()[3].clone())),
                    ];
                    let constraint = Box::new(Constraint::new_with_pattern(pattern));

                    let constrained_iterator =
                        NBestIterator::new(&lattice, eos_node.clone(), constraint);

                    assert_eq!(constrained_iterator.collect::<Vec<_>>().len(), 9);
                }
            }
        }
    }

    mod cap {
        use super::*;

        #[test]
        fn new() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let _cap = Cap::new(nodes, 24, 42);
        }

        #[test]
        fn ord() {
            let preceding_edge_costs1 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node1 = Node::eos(1, preceding_edge_costs1, 5, 42);
            let nodes1 = vec![node1];
            let cap1 = Cap::new(nodes1, 24, 42);

            let preceding_edge_costs2 = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node2 = Node::eos(1, preceding_edge_costs2, 5, 42);
            let nodes2 = vec![node2];
            let cap2 = Cap::new(nodes2, 24, 42);

            let preceding_edge_costs3 = Rc::new(vec![2, 7, 1, 8, 2, 8]);
            let node3 = Node::eos(2, preceding_edge_costs3, 3, 31);
            let nodes3 = vec![node3];
            let cap3 = Cap::new(nodes3, 12, 4242);

            assert!(cap1 == cap2);
            assert!(cap1 < cap3);
        }

        #[test]
        fn tail_path() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs.clone(), 5, 42);
            let nodes = vec![node];
            let cap = Cap::new(nodes, 24, 42);

            assert_eq!(cap.tail_path().len(), 1);
            assert_eq!(
                cap.tail_path()[0].preceding_edge_costs(),
                preceding_edge_costs.as_slice()
            );
        }

        #[test]
        fn tail_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = Cap::new(nodes, 24, 42);

            assert_eq!(cap.tail_path_cost(), 24);
        }

        #[test]
        fn whole_path_cost() {
            let preceding_edge_costs = Rc::new(vec![3, 1, 4, 1, 5, 9, 2, 6]);
            let node = Node::eos(1, preceding_edge_costs, 5, 42);
            let nodes = vec![node];
            let cap = Cap::new(nodes, 24, 42);

            assert_eq!(cap.whole_path_cost(), 42);
        }
    }
}
