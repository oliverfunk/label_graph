pub trait LabelNode {
    type Data;

    fn new_node(label: String) -> Self;
    fn get_label(&self) -> String;
    fn set_data(&mut self, data: Self::Data);
    fn get_data(&self) -> Option<&Self::Data>;
    fn attach_connection(&mut self, edge: GraphEdge);
}

pub trait DirectedNode {
    fn link_node_as_input(&mut self, from_node: mut Self);

    fn link_node_as_output(&mut self, to_node: mut Self);

    /// All nodes that are input into this node
    fn get_input_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;

    /// All nodes that this node outputs to
    fn get_output_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;
}

pub trait UndirectedNode {
    fn get_connected_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;
}

/// Holds the nodes of the graph
#[derive(Debug)]
pub struct LabelGraph<N>
where
    N: LabelNode,
{
    nodes: Vec<N>,
    sorted: bool,
}

//impl<N> LabelGraph<N>
//where
//    N: LabelNode + UndirectedNode,
//{
//    pub fn link_nodes(&self, from: String, to: String) {
//
//    }
//}
//
//impl<N> LabelGraph<N>
//where
//    N: LabelNode + DirectedNode,
//{
//    pub fn link_nodes(&mut self, from_node_label: &str, to_node_label: &str, weight: i32) {
//        if !self.sorted { panic!("Cannot link before sorting") }
//
//        // check if the nodes exist
//        let from_n = self.get_node_index(from_node_label);
//        let to_n = self.get_node_index(to_node_label);
//
//        if from_n.is_some() && to_n.is_some() {
//            let edge = GraphEdge::new(from_node_label.to_string(), to_node_label.to_string(), weight);
//
//            self.nodes.get_mut(from_n.unwrap().0).unwrap().attach_connection()
//        }
//    }
//}

impl<N> LabelGraph<N>
where
    N: LabelNode,
{
    pub fn new() -> LabelGraph<N> {
        LabelGraph {
            nodes: Vec::new(),
            sorted: false,
        }
    }

    pub fn with_capacity(capacity: usize) -> LabelGraph<N> {
        LabelGraph {
            nodes: Vec::with_capacity(capacity),
            sorted: false,
        }
    }

    pub fn add_node(&mut self, node: N) {
        // can only add nodes if not sorted, becuase once it is sorted
        // one may link nodes and the links depend on the index
        // therefore the indecies cannot change
        //        if !self.sorted {
        //            // dono what happens if two nodes have the same label
        //            self.nodes.push(node);
        //            self.sorted = false;
        //        }

        self.nodes.push(node);
        self.sorted = false;
    }

    pub fn get_node_index(&self, label: &str) -> Option<NodeIndex> {
        if self.sorted {
            match self
                .nodes
                .binary_search_by_key(&label.to_string(), |n| n.get_label())
            {
                Ok(idx) => Some(NodeIndex(idx)),
                Err(_idx) => None,
            }
        } else {
            panic!("Cannot search for a node without sorting first!")
        }
    }

    pub fn get_node(&self, label: &str) -> Option<(&N, NodeIndex)> {
        if self.sorted {
            match self
                .nodes
                .binary_search_by_key(&label.to_string(), |n| n.get_label())
            {
                Ok(idx) => Some((self.nodes.get(idx).unwrap(), NodeIndex(idx))),
                Err(_idx) => None,
            }
        } else {
            panic!("Cannot search for a node without sorting first!")
        }
    }

    pub fn get_mut_node(&mut self, label: &str) -> Option<(&mut N, NodeIndex)> {
        if self.sorted {
            match self
                .nodes
                .binary_search_by_key(&label.to_string(), |n| n.get_label())
            {
                Ok(idx) => Some((self.nodes.get_mut(idx).unwrap(), NodeIndex(idx))),
                Err(_idx) => None,
            }
        } else {
            panic!("Cannot search for a node without sorting first!")
        }
    }

    pub fn sort(&mut self) {
        self.nodes.sort_unstable_by_key(|n| n.get_label());
        self.sorted = true;
    }
}

/// Node to be used in a directed graph
#[derive(Debug)]
pub struct DirectedLabelNode<D> {
    label: String,
    connections: Vec<GraphEdge>,
    data: Option<Box<D>>,
}

impl<D> DirectedNode for DirectedLabelNode<D> {
    fn link_node_as_input(&mut self, from_node: mut DirectedLabelNode<D>, weight: i32) {
        self.attach_connection(GraphEdge::new(from_node.get_label(), self.get_label(), weight));
        from_node.link_node_as_output(self, weight);
    }

    fn link_node_as_output(&mut self, to_node: DirectedLabelNode<D>) {

    }

    fn get_input_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge> {
        self.connections
            .iter()
            .map(|edge| edge.clone())
            .filter(|edge| edge.to == this_index)
            .collect()
    }

    fn get_output_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge> {
        self.connections
            .iter()
            .map(|edge| edge.clone())
            .filter(|c| c.from == this_index)
            .collect()
    }
}

impl<D> LabelNode for DirectedLabelNode<D> {
    type Data = D;

    fn new_node(label: String) -> Self {
        Self {
            label,
            connections: Vec::new(),
            data: None,
        }
    }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn set_data(&mut self, data: D) {
        let d = Box::new(data);
        self.data = Some(d);
    }

    fn get_data(&self) -> Option<&D> {
        match self.data {
            Some(ref d) => Some(d),
            None => None,
        }
    }

    fn attach_connection(&mut self, edge: GraphEdge) {
        self.connections.push(edge);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeIndex(usize);

#[derive(Debug, Clone)]
pub struct GraphEdge {
    from: String,
    to: String,
    weight: i32,
}

impl GraphEdge {
    pub fn new(from: String, to: String, weight: i32) -> GraphEdge {
        GraphEdge { from, to, weight }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
