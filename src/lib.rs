pub trait LabelGraph<D> {
    /// A helper function used to wrap a panic!() if the
    /// graph isn't sorted when it needs to be.
    fn check_sorted(&self, error_message: &str);

    /// Sort the graph, based on the node label
    fn sort(&mut self);

    /// Add a node to the graph. The graph
    /// will no longer be sorted if a node is added.
    fn add_node(&mut self, node: LabelNode<D>);

    /// Get a non-mutable reference to a node.
    fn get_node(&self, node_label: &str) -> Option<&LabelNode<D>>;

    /// Get a mutable reference to a node.
    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>>;

    /// Check if a node with the node_label exists in the graph
    fn check_node_exists(&self, node_label: &str) -> bool;
}

pub trait DirectedGraph {
    /// Create a link from one node to another (directional). If the link already exists
    /// the weight will be updated.
    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i32);

    /// Return all nodes that input into this node
    fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;

    /// Return all nodes that this node outputs to
    fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
}

pub trait UndirectedGraph {
    /// Create a link between node A and node B (non-directional). If the link already exists
    /// the weight will be updated.
    fn link_nodes(&mut self, node_a: &str, node_b: &str, weight: i32);

    /// Return all nodes connected to this node
    fn get_connected_nodes(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
}

/// Holds the nodes of the graph
#[derive(Debug)]
pub struct DirectedLabelGraph<D> {
    nodes: Vec<LabelNode<D>>,
    sorted: bool,
}

impl<D> DirectedLabelGraph<D> {
    pub fn new() -> DirectedLabelGraph<D> {
        DirectedLabelGraph {
            nodes: Vec::new(),
            sorted: false,
        }
    }

    pub fn with_capacity(capacity: usize) -> DirectedLabelGraph<D> {
        DirectedLabelGraph {
            nodes: Vec::with_capacity(capacity),
            sorted: false,
        }
    }
}

impl<D> LabelGraph<D> for DirectedLabelGraph<D> {
    fn check_sorted(&self, error_message: &str) {
        if !self.sorted {
            panic!(error_message.to_string());
        }
    }

    fn sort(&mut self) {
        self.nodes.sort_unstable_by_key(|n| n.get_label());
        self.sorted = true;
    }

    fn add_node(&mut self, node: LabelNode<D>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn get_node(&self, node_label: &str) -> Option<&LabelNode<D>> {
        self.check_sorted("Cannot search for a node without sorting first!");

        match self
            .nodes
            .binary_search_by_key(&node_label.to_string(), |n| n.get_label())
        {
            Ok(idx) => Some(self.nodes.get(idx).unwrap()),
            Err(_idx) => None,
        }
    }

    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>> {
        self.check_sorted("Cannot search for a node without sorting first!");

        match self
            .nodes
            .binary_search_by_key(&node_label.to_string(), |n| n.get_label())
        {
            Ok(idx) => Some(self.nodes.get_mut(idx).unwrap()),
            Err(_idx) => None,
        }
    }

    fn check_node_exists(&self, node_label: &str) -> bool {
        match self.get_node(node_label) {
            Some(_) => true,
            None => false,
        }
    }
}

impl<D> DirectedGraph for DirectedLabelGraph<D> {
    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i32) {
        self.check_sorted("Cannot link nodes before sorting!");

        // TODO: must be a better way to check if both nodes exsits without doing the lookup twice
        // once to check and once to mutate.
        if !self.check_node_exists(from_node_label) || !self.check_node_exists(to_node_label) {
            return;
        }

        self.get_mut_node(from_node_label).map(|n| {
            // have to do it this way, don't want to use nightly NLL feature.
            let updated = match n.connections.iter_mut().find(|n| {
                n.node_label == to_node_label.to_string() && n.direction == ConnectionDirection::To
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

        self.get_mut_node(to_node_label).map(|n| {
            let updated = match n.connections.iter_mut().find(|n| {
                n.node_label == from_node_label.to_string() && n.direction == ConnectionDirection::From
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

    fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>> {
        self.check_sorted("Cannot check inputs for node before sorting!");

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

    fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>> {
        self.check_sorted("Cannot check outputs for node before sorting!");

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
    weight: i32,
}

impl GraphEdge {
    pub fn new(node_label: String, direction: ConnectionDirection, weight: i32) -> GraphEdge {
        GraphEdge {
            direction,
            node_label,
            weight,
        }
    }
}

/// A node in the graph. It holds its label, connections and data.
#[derive(Debug)]
pub struct LabelNode<D> {
    label: String,
    connections: Vec<GraphEdge>,
    data: D,
}

impl<D> LabelNode<D> {
    pub fn new_node(label: &str, data: D) -> Self {
        Self {
            label: label.to_string(),
            connections: Vec::new(),
            data,
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_data(&self) -> &D {
        &self.data
    }

    pub fn get_mut_data(&mut self) -> &mut D {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
