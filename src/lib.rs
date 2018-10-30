#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConnectionDirection {
    From,
    To,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    node_label: String,
    direction: ConnectionDirection,
    weight: i32,
}

impl GraphEdge {
    pub fn new(node_label: String, direction: ConnectionDirection, weight: i32) -> GraphEdge {
        GraphEdge {
            node_label,
            direction,
            weight,
        }
    }
}

#[derive(Debug)]
pub struct LabelNode<D> {
    label: String,
    connections: Vec<GraphEdge>,
    data: Option<D>,
}

impl<D> LabelNode<D> {
    pub fn new_node(label: &str) -> Self {
        Self {
            label: label.to_string(),
            connections: Vec::new(),
            data: None,
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn set_data(&mut self, data: D) {
        self.data = Some(data);
    }

    pub fn get_data(&self) -> Option<&D> {
        match self.data {
            Some(ref d) => Some(d),
            None => None,
        }
    }
}

pub trait DirectedGraph {
    /// Create a link from one node to another (directional)
    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str);

    /// Return all nodes that input into this node
    fn get_inputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;

    /// Return all nodes that this node outputs to
    fn get_outputs_for_node(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
}

pub trait UndirectedGraph {
    /// Create a link between node A and node B (non-directional)
    fn link_nodes(&mut self, node_a: &str, node_b: &str);

    /// Return all nodes connected to this node
    fn get_connected_nodes(&self, node_label: &str) -> Option<Vec<GraphEdge>>;
}

pub trait LabelGraph<D> {
    fn sort(&mut self);
    fn check_sorted(&self, error_message: &str);
    fn add_node(&mut self, node: LabelNode<D>);
    fn get_node(&self, node_label: &str) -> Option<&LabelNode<D>>;
    fn get_mut_node(&mut self, node_label: &str) -> Option<&mut LabelNode<D>>;
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
    fn sort(&mut self) {
        self.nodes.sort_unstable_by_key(|n| n.get_label());
        self.sorted = true;
    }

    fn check_sorted(&self, error_message: &str) {
        if !self.sorted {
            panic!(error_message.to_string());
        }
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
}

impl<D> DirectedGraph for DirectedLabelGraph<D> {
    fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str) {
        self.check_sorted("Cannot link nodes before sorting!");

        self.get_mut_node(from_node_label).map(|n| {
            let edge = GraphEdge::new(to_node_label.to_string(), ConnectionDirection::To, 1);
            n.connections.push(edge)
        });

        self.get_mut_node(to_node_label).map(|n| {
            let edge = GraphEdge::new(from_node_label.to_string(), ConnectionDirection::From, 1);
            n.connections.push(edge)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
