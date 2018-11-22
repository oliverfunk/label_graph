extern crate labelgraph;

use labelgraph::*;

fn main() {
    let mut g = DirectedLabelGraph::new();

    let n1 = LabelGraphNode::new(String::new());
    let n2 = LabelGraphNode::new(String::new());
    let n3 = LabelGraphNode::new(String::new());

    g.add_node("n1", n1);
    g.add_node("n2", n2);
    g.add_node("n3", n3);

    println!("linking n1 -> n2");
    g.link_nodes("n1", "n2", 1);
    g.link_nodes("node1", "node22", 2); // won't work

    {
        // modify n1's data
        let n1_data = g.get_mut_node_data("n1").unwrap();
        n1_data.push_str("hello world");
    }
    println!("n1's data: {:?}", g.get_node_data("n1"));

    for n in g.iter_nodes() {
        println!("node: {:?}", n)
    }

    println!("node1's outputs: {:?}", g.get_inputs_for_node("n1"));
    println!("node2's inputs: {:?}", g.get_outputs_for_node("n1"));
}
