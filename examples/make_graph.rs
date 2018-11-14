extern crate labelgraph;

use labelgraph::*;

type NodeData = (String);

fn main() {
    let mut g = DirectedLabelGraph::<NodeData>::new();

    let n1 = LabelNode::new_node("node1", String::new());
    let n2 = LabelNode::new_node("node2", String::new());
    let n3 = LabelNode::new_node("node3", String::new());

    g.add_node(n1);
    g.add_node(n2);
    g.add_node(n3);

    g.sort();

    println!("linking node1 -> node2");
    g.link_nodes("node1", "node2", 1);
    g.link_nodes("node1", "node22", 2); // won't work

    {
        let n1_from_graph = g.get_mut_node("node1").unwrap();
        {
            let d = n1_from_graph.get_mut_data();
            d.clear();
            d.push_str("HELLO WORLD");
        }
        println!("node1's data: {:?}", n1_from_graph.get_data());
    }

    for n in g.all_nodes(){
        println!("node: {:?}", n)
    }


    println!("node1's outputs: {:?}", g.get_inputs_for_node("node1"));
    println!("node2's inputs: {:?}", g.get_outputs_for_node("node2"));
}
