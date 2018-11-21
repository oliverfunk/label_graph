use std::borrow::{Borrow, BorrowMut};
use std::slice::*;

use im::ordmap::Entry;
use im::OrdMap;

//pub trait LabelGraph<D> {
//        /// Add a node to the graph. The graph
//    /// will no longer be sorted if a node is added.
//    fn add_node(&mut self, label: String, node: LabelNode<D>);
//
//    /// Get a non-mutable reference to a node.
//    fn get_node(&self, node_label: &str) -> Option<&LabelNode<D>>;
//
//    /// Get a mutable reference to a node.
//    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>>;
//
//    /// Check if a node with the node_label exists in the graph
//    fn check_node_exists(&self, node_label: &str) -> bool;
//}
//
//pub trait DirectedGraph {
//    /// Create a link from one node to another (directional). If the link already exists
//    /// the weight will be updated.
//    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i64);
//
//    /// Return all nodes that input into this node
//    fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
//
//    /// Return all nodes that this node outputs to
//    fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
//}
//
//pub trait UndirectedGraph {
//    /// Create a link between node A and node B (non-directional). If the link already exists
//    /// the weight will be updated.
//    fn link_nodes(&mut self, node_a: &str, node_b: &str, weight: i64);
//
//    /// Return all nodes connected to this node
//    fn get_connected_nodes(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
//}

/// Holds the nodes of the graph
#[derive(Debug, Clone)]
pub struct DirectedLabelGraph<D>
where
    D: Clone,
{
    nodes: OrdMap<String, LabelNode<D>>,
}

impl<'a, D: Clone> DirectedLabelGraph<D> {
    pub fn new() -> Self {
        DirectedLabelGraph {
            nodes: OrdMap::new(),
        }
    }

    fn node_exists(&self, node_label: &str) -> bool {
        self.nodes.contains_key(node_label)
    }

    fn add_node(&mut self, node_label: &str, node: LabelNode<D>) {
        self.nodes.insert(node_label.to_string(), node);
    }

    fn get_node(&mut self, node_label: &str) -> Option<&'a LabelNode<D>> {
        let a = self.nodes.entry(node_label.to_string());
        match a {
            Entry::Occupied(oe) => Some(oe.get()),
            Entry::Vacant(_) => None,
        }
    }

    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>> {
        let a = self.nodes.entry(node_label.to_string());
        match a {
            Entry::Occupied(mut oe) => Some(oe.get_mut()),
            Entry::Vacant(_) => None,
        }
    }
    //
    //    fn get_mut_node_data(&mut self, node_label: &str) -> &'a mut D {
    //        Box::leak(self.nodes.get(node_label).unwrap().clone().data)
    //    }

    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i64) {
        let mut from_exists = false;

        self.get_mut_node(from_node_label).map(|n| {
            from_exists = true;

            // have to do it this way, don't want to use nightly NLL feature.
            let updated = match n.connections.iter_mut().find(|edge| {
                edge.node_label == to_node_label.to_string()
                    && edge.direction == ConnectionDirection::To
            }) {
                Some(e) => {
                    e.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                n.connections.push(GraphEdge::new(
                    to_node_label.to_string(),
                    ConnectionDirection::To,
                    weight,
                ))
            }
        });

        // if from doesn't exist, return and don't modify to
        if !from_exists { return; }

        self.get_mut_node(to_node_label).map(|n| {
            let updated = match n.connections.iter_mut().find(|edge| {
                edge.node_label == from_node_label.to_string()
                    && edge.direction == ConnectionDirection::From
            }) {
                Some(e) => {
                    e.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                n.connections.push(GraphEdge::new(
                    from_node_label.to_string(),
                    ConnectionDirection::From,
                    weight,
                ))
            }
        });
    }

    fn get_inputs_for_node(&mut self, node_label: &str) -> Option<Vec<GraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .map(|edge| edge.clone())
                    .filter(|edge| edge.direction == ConnectionDirection::From)
                    .collect(),
            )
        } else {
            None
        }
    }

    fn get_outputs_for_node(&mut self, node_label: &str) -> Option<Vec<GraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .map(|edge| edge.clone())
                    .filter(|edge| edge.direction == ConnectionDirection::To)
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
#[derive(Debug, Clone)]
pub struct GraphEdge {
    direction: ConnectionDirection,
    node_label: String,
    weight: i64,
}

impl GraphEdge {
    pub fn new(node_label: String, direction: ConnectionDirection, weight: i64) -> GraphEdge {
        GraphEdge {
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

/// A node in the graph. It holds its label, connections and data.
#[derive(Debug, Clone)]
pub struct LabelNode<D> {
    connections: Vec<GraphEdge>,
    data: Box<D>,
}

impl<D: Clone> LabelNode<D> {
    pub fn new_node(label: &str, data: D) -> Self {
        Self {
            connections: Vec::new(),
            data: Box::new(data),
        }
    }
}
