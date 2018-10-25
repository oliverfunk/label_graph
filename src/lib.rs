/// Directed graph store
pub struct DirectedGraph<L, D>
where L: Ord
{
    store: Vec<DirectedGraphNode<L, D>>,
    stored: bool,
}

/// Node to be used in a directed graph
pub struct DirectedGraphNode<L, D>
where L: Ord
{
    label: L,
    connections: Vec<GraphEdge>,
    data: D,
}

#[derive(Debug, Copy, Clone)]
pub struct NodeIndex {
    index: usize,
}

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
