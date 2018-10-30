extern crate labelgraph;

use labelgraph::*;

fn main() {
    let mut g = LabelGraph::<DirectedLabelNode<Vec<i32>>>::new();
    let n1 = DirectedLabelNode::<Vec<i32>>::new_node("test1".to_string());
    let n2 = DirectedLabelNode::<Vec<i32>>::new_node("test2".to_string());
    let n3 = DirectedLabelNode::<Vec<i32>>::new_node("test3".to_string());

    g.add_node(n1);
    g.add_node(n2);
    g.add_node(n3);

    g.sort();

    //    let (n1_from_graph, n1_idx) = g.get_mut_node("test1").unwrap();
    //    let (n2_from_graph, n2_idx) = g.get_node("test2").unwrap();

    //    println!("{:?}", n1_from_graph.get_label());
}
