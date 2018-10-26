pub trait LabelNode {
    type Label;
    type Data;

    fn new_node(label: Self::Label, data: Self::Data) -> Self;
    fn get_label(&self) -> Self::Label;
    fn attach_connection(&mut self, edge: GraphEdge);
}

pub trait DirectedNode {
    /// All nodes that are input into this node
    fn input_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;

    /// All nodes that this node outputs to
    fn output_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;
}

pub trait UndirectedNode {
    fn get_connected_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge>;
}

/// Holds the nodes of the graph
pub struct LabelGraph<N>
where
    N: LabelNode,
{
    nodes: Vec<N>,
    sorted: bool,
}

impl<N> LabelGraph<N> {
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

    }

    pub fn get_node(&mut self, )
}


/// Node to be used in a directed graph
pub struct DirectedGraphNode<L, D>
where
    L: Ord + Clone,
{
    label: L,
    connections: Vec<GraphEdge>,
    data: D,
}

impl<L, D> DirectedNode for DirectedGraphNode<L, D>
where
    L: Ord + Clone,
{
    fn input_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge> {
        self.connections
            .iter()
            .map(|edge| edge.clone())
            .filter(|edge| edge.to == this_index)
            .collect()
    }

    fn output_nodes(&self, this_index: NodeIndex) -> Vec<GraphEdge> {
        self.connections
            .iter()
            .map(|edge| edge.clone())
            .filter(|c| c.from == this_index)
            .collect()
    }
}

impl<L, D> LabelNode for DirectedGraphNode<L, D>
where
    L: Ord + Clone,
{
    type Label = L;
    type Data = D;

    fn new_node(label: Self::Label, data: Self::Data) -> Self {
        Self {
            label,
            connections: Vec::new(),
            data,
        }
    }

    fn get_label(&self) -> L {
        self.label.clone()
    }

    fn attach_connection(&mut self, edge: GraphEdge) {
        self.connections.push(edge);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeIndex {
    index: usize,
}

#[derive(Clone)]
pub struct GraphEdge {
    from: NodeIndex,
    to: NodeIndex,
    weight: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
