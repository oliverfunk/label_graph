use std::collections::btree_map::Values;
use std::collections::BTreeMap;

/// Holds the nodes of the graph
#[derive(Debug, Clone)]
pub struct DirectedLabelGraph<D>
{
    nodes: BTreeMap<String, LabelGraphNode<D>>,
}

impl<D> Default for DirectedLabelGraph<D> {
    fn default() -> Self {
        DirectedLabelGraph {
            nodes: BTreeMap::new(),
        }
    }
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

    pub fn iter_nodes(&self) -> Values<String, LabelGraphNode<D>> {
        self.nodes.values()
    }

    pub fn check_node_exists(&self, node_label: &str) -> bool {
        self.nodes.contains_key(node_label)
    }

    pub fn add_node(&mut self, node_label: &str, node: LabelGraphNode<D>) {
        self.nodes.insert(node_label.to_string(), node);
    }

    pub fn get_node_data(&self, node_label: &str) -> Option<&D> {
        self.get_node(node_label).map(|n| &n.data)
    }

    pub fn get_mut_node_data(&mut self, node_label: &str) -> Option<&mut D> {
        self.get_mut_node(node_label).map(|n| &mut n.data)
    }

    pub fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i64) {
        if !(self.check_node_exists(from_node_label) && self.check_node_exists(to_node_label)) {
            return;
        }

        if let Some(node) = self.get_mut_node(from_node_label) {
            // correct way to do this is the following,
            // but waiting for nightly feature NLL (non-lexical lifetimes)
            // to go stable
//            match node.connections.iter_mut().find(|edge| {
//                edge.node_label == to_node_label && edge.direction == ConnectionDirection::To
//            }) {
//                Some(edge) => {
//                    edge.weight = weight;
//                    true
//                }
//                None => {
//                    // create a new edge and push it
//                    node.connections.push(GraphEdge::new(
//                        to_node_label.to_string(),
//                        ConnectionDirection::To,
//                        weight,
//                    ))
//                }
//            };

            let updated = match node.connections.iter_mut().find(|edge| {
                edge.node_label == to_node_label && edge.direction == ConnectionDirection::To
            }) {
                Some(edge) => {
                    edge.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                node.connections.push(LabelGraphEdge::new(
                    to_node_label.to_string(),
                    ConnectionDirection::To,
                    weight,
                ))
            }
        }

        if let Some(node) = self.get_mut_node(to_node_label) {
            let updated = match node.connections.iter_mut().find(|edge| {
                edge.node_label == from_node_label && edge.direction == ConnectionDirection::From
            }) {
                Some(edge) => {
                    edge.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                node.connections.push(LabelGraphEdge::new(
                    from_node_label.to_string(),
                    ConnectionDirection::From,
                    weight,
                ))
            }
        };
    }

    pub fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<LabelGraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .filter(|edge| edge.direction == ConnectionDirection::From)
                    .cloned()
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<LabelGraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .filter(|edge| edge.direction == ConnectionDirection::To)
                    .cloned()
                    .collect(),
            )
        } else {
            None
        }
    }
}

/// Edge direction.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConnectionDirection {
    From,
    To,
}

/// Representation of a an edge between two nodes.
// todo: try convert node_label property into node and store a reference to a node.
// Will need to have some method that drops the ref if the node is deleted
#[derive(Debug, Clone)]
pub struct LabelGraphEdge {
    direction: ConnectionDirection,
    node_label: String,
    weight: i64,
}

impl LabelGraphEdge {
    pub fn new(node_label: String, direction: ConnectionDirection, weight: i64) -> LabelGraphEdge {
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
pub struct LabelGraphNode<D>
{
    connections: Vec<LabelGraphEdge>,
    data: D,
}

impl<D> LabelGraphNode<D> {
    pub fn new(data: D) -> Self {
        LabelGraphNode {
            connections: Vec::new(),
            data
        }
    }
}
