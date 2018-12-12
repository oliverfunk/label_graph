mod graph;
mod graph_traversal_algorithms;

pub use crate::{graph::DirectedLabelGraph, graph_traversal_algorithms::*};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
