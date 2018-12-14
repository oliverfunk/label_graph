//use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::iter::FromIterator;

/// Holds the nodes of the graph
#[derive(Debug)]
pub struct DirectedLabelGraph<D> {
    nodes: BTreeMap<String, LabelGraphNode<D>>,
}

impl<D> DirectedLabelGraph<D> {
    pub fn new() -> Self {
        DirectedLabelGraph::default()
    }

    fn get_node(&self, node_label: &str) -> Option<&LabelGraphNode<D>> {

        self.nodes.get(node_label)
    }

    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelGraphNode<D>> {
        self.nodes.get_mut(node_label)
    }

    pub fn check_node_exists(&self, node_label: &str) -> bool {
        self.nodes.contains_key(node_label)
    }

    // update node data if it exsits
    pub fn create_node(&mut self, node_label: &str, node_data: D) {
        //        self.nodes
        //            .entry(node_label.to_string())
        //            .and_modify(|node| node.data = node_data)
        //            .or_insert(LabelGraphNode::new(node_data));

        if self.check_node_exists(node_label) {
            self.nodes.get_mut(node_label).unwrap().data = node_data;
        } else {
            self.nodes
                .insert(node_label.to_string(), LabelGraphNode::new(node_data));
        }
    }

    pub fn get_node_data(&self, node_label: &str) -> Option<&D> {
        self.get_node(node_label).map(|n| &n.data)
    }

    pub fn get_mut_node_data(&mut self, node_label: &str) -> Option<&mut D> {
        self.get_mut_node(node_label).map(|n| &mut n.data)
    }

    pub fn unlink_nodes(&mut self, node_label_A: &str, _node_label_B: &str) {
        if let Some(_node) = self.get_mut_node(node_label_A) {}
    }

    pub fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i64) {
        if !(self.check_node_exists(from_node_label) && self.check_node_exists(to_node_label)) {
            return;
        }

        if let Some(node) = self.get_mut_node(from_node_label) {
            match node.connections.iter_mut().find(|edge| {
                edge.node_label == to_node_label && edge.direction == ConnectionDirection::To
            }) {
                Some(edge) => {
                    edge.weight = weight;
                }
                None => {
                    // create a new edge and push it
                    node.connections.push(LabelGraphEdge::new(
                        to_node_label.to_string(),
                        ConnectionDirection::To,
                        weight,
                    ))
                }
            };
        }

        if let Some(node) = self.get_mut_node(to_node_label) {
            match node.connections.iter_mut().find(|edge| {
                edge.node_label == from_node_label && edge.direction == ConnectionDirection::From
            }) {
                Some(edge) => {
                    edge.weight = weight;
                }
                None => {
                    // create a new edge and push it
                    node.connections.push(LabelGraphEdge::new(
                        from_node_label.to_string(),
                        ConnectionDirection::From,
                        weight,
                    ))
                }
            };
        };
    }

    pub fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<&LabelGraphEdge>> {
        if let Some(node) = self.get_node(node_label) {
            Some(
                node.connections
                    .iter()
                    .filter(|edge| edge.direction == ConnectionDirection::From)
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<&LabelGraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .filter(|edge| edge.direction == ConnectionDirection::To)
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn iter_node_data(&self) -> impl Iterator<Item=&D> {
        IterGraphNodeData::new(self.nodes.values().map(|node| &node.data).collect())
    }

    pub fn iter_node_label_and_data(&self) -> impl Iterator<Item=(&String, &D)> {
        IterGraphNodeLabelAndData::new(self.nodes.iter().map(|node| (node.0, &node.1.data)).collect())
    }
}

impl<D> Default for DirectedLabelGraph<D> {
    fn default() -> Self {
        DirectedLabelGraph {
            nodes: BTreeMap::new(),
        }
    }
}

pub struct IterGraphNodeData<'a, D> {
    curr_idx: usize,
    items: Vec<&'a D>,
}

impl<'a, D> IterGraphNodeData<'a, D> {
    pub fn new(items: Vec<&'a D>) -> Self {
        IterGraphNodeData {
            curr_idx: 0,
            items,
        }
    }
}

impl<'a, D> Iterator for IterGraphNodeData<'a, D> {
    type Item = &'a D;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.curr_idx >= self.items.len(){
            None
        } else {
            self.curr_idx += 1;
            Some(self.items[self.curr_idx - 1])
        }
    }
}

pub struct IterGraphNodeLabelAndData<'a, D> {
    curr_idx: usize,
    items: Vec<(&'a String, &'a D)>,
}

impl<'a, D> IterGraphNodeLabelAndData<'a, D> {
    pub fn new(items: Vec<(&'a String, &'a D)>) -> Self {
        IterGraphNodeLabelAndData {
            curr_idx: 0,
            items,
        }
    }
}

impl<'a, D> Iterator for IterGraphNodeLabelAndData<'a, D> {
    type Item = (&'a String, &'a D);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.curr_idx >= self.items.len(){
            None
        } else {
            self.curr_idx += 1;
            Some(self.items[self.curr_idx - 1])
        }
    }
}

/// Representation of a an edge between two nodes.
#[derive(Debug, Clone)]
pub struct LabelGraphEdge {
    direction: ConnectionDirection,
    node_label: String,
    weight: i64,
}

/// Edge direction.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConnectionDirection {
    From,
    To,
}

impl LabelGraphEdge {
    fn new(node_label: String, direction: ConnectionDirection, weight: i64) -> LabelGraphEdge {
        LabelGraphEdge {
            direction,
            node_label,
            weight,
        }
    }

    pub fn get_connection_direction(&self) -> ConnectionDirection {
        self.direction
    }

    pub fn get_connected_label(&self) -> String {
        self.node_label.clone()
    }

    pub fn get_edge_weight(&self) -> i64 {
        self.weight
    }
}

/// A node in the graph, which stores its data and its connections to other nodes.
#[derive(Debug, Clone)]
struct LabelGraphNode<D> {
    connections: Vec<LabelGraphEdge>,
    data: D,
}

impl<D> LabelGraphNode<D> {
    pub fn new(data: D) -> Self {
        LabelGraphNode {
            connections: Vec::new(),
            data,
        }
    }
}
