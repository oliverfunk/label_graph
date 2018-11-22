use std::collections::BTreeMap;
use std::collections::btree_map::Values;

/// Holds the nodes of the graph
#[derive(Debug, Clone)]
pub struct DirectedLabelGraph<D>
{
    nodes: BTreeMap<String, LabelNode<D>>,
}

impl<D> DirectedLabelGraph<D> {
    pub fn new() -> Self {
        DirectedLabelGraph {
            nodes: BTreeMap::new()
        }
    }

    fn get_node(&self, node_label: &str) -> Option<&LabelNode<D>> {
        self.nodes.get(node_label)
    }

    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>> {
        self.nodes.get_mut(node_label)
    }

    pub fn iter_nodes(&self) -> Values<String, LabelNode<D>> {
        self.nodes.values()
    }

    pub fn node_exists(&self, node_label: &str) -> bool {
        self.nodes.contains_key(node_label)
    }

    pub fn add_node(&mut self, node_label: &str, node: LabelNode<D>) {
        self.nodes.insert(node_label.to_string(), node);
    }

    pub fn get_node_data(&self, node_label: &str) -> Option<&D> {
        self.get_node(node_label).map(|n| &n.data)
    }

    pub fn get_mut_node_data(&mut self, node_label: &str) -> Option<&mut D> {
        self.get_mut_node(node_label).map(|n| &mut n.data)
    }

    pub fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i64) {
        let mut from_exists = false;

        self.get_mut_node(from_node_label).map(|node| {
            from_exists = true;

            // have to do it this way, don't want to use nightly NLL feature.
            let updated = match node.connections.iter_mut().find(|edge| {
                edge.node_label == to_node_label.to_string()
                    && edge.direction == ConnectionDirection::To
            }) {
                Some(edge) => {
                    edge.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                node.connections.push(GraphEdge::new(
                    to_node_label.to_string(),
                    ConnectionDirection::To,
                    weight,
                ))
            }
        });

        // if from doesn't exist, return and don't modify to_node
        if !from_exists { return; }

        self.get_mut_node(to_node_label).map(|node| {
            let updated = match node.connections.iter_mut().find(|edge| {
                edge.node_label == from_node_label.to_string()
                    && edge.direction == ConnectionDirection::From
            }) {
                Some(edge) => {
                    edge.weight = weight;
                    true
                }
                None => false,
            };

            if !updated {
                // create a new edge and push it
                node.connections.push(GraphEdge::new(
                    from_node_label.to_string(),
                    ConnectionDirection::From,
                    weight,
                ))
            }
        });
    }

    pub fn get_inputs_for_node(&mut self, node_label: &str) -> Option<Vec<GraphEdge>> {
        let node = self.get_node(node_label);
        if node.is_some() {
            Some(
                node.unwrap()
                    .connections
                    .iter()
                    .filter(|edge| edge.direction == ConnectionDirection::From)
                    .map(|edge| edge.clone())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_outputs_for_node(&mut self, node_label: &str) -> Option<Vec<GraphEdge>> {
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
// todo: convert node_label property into node and store a reference to a node. Will need to have some method that drops the ref if the node is deleted
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

/// A node in the graph, which stores its data and its connections to other nodes.
#[derive(Debug, Clone)]
pub struct LabelNode<D> {
    connections: Vec<GraphEdge>,
    data: D,
}

// todo: is it good to be explicate about the Clone trait bound in this struct?
impl<D> LabelNode<D> {
    pub fn new(data: D) -> Self {
        LabelNode {
            connections: Vec::new(),
            data
        }
    }
}
