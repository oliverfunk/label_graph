extern crate labelgraph;

use labelgraph::*;

type NodeData = (String, Vec<i32>);

fn main() {
    let mut g = DirectedLabelGraph::<NodeData>::new();

    let n1 = LabelNode::new_node("node1");
    let n2 = LabelNode::new_node("node2");
    let n3 = LabelNode::new_node("node3");

    g.add_node(n1);
    g.add_node(n2);
    g.add_node(n3);

    g.sort();

    println!("linking node1 -> node2");
    g.link_nodes("node1", "node2", 1);
    g.link_nodes("node1", "node22", 2);

    {
        let n1_from_graph = g.get_mut_node("node1").unwrap();

        let data = ("DATA".to_string(), vec![100, 53, 23]);

        n1_from_graph.set_data(data);

        println!("node1's data: {:?}", n1_from_graph.get_data().unwrap());
    }

    println!("node1's outputs: {:?}", g.get_inputs_for_node("node1"));
    println!("node2's inputs: {:?}", g.get_outputs_for_node("node2"));
}
