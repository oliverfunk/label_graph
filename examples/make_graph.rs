//
//
//
//#[derive(Debug)]
//struct NodeData(i32);
//
//
//
fn main() {
    print!("a");
}
//    let mut g =
//
//    let n1 = NodeData(2);
//    let n2 = NodeData(3);
//    let n3 = NodeData(4);
//
//    g.create_node("n1", n1);
//    g.create_node("n2", n2);
//    g.create_node("n3", n3);
//
//    println!("linking: n1 -> n2");
//    g.link_nodes("n1", "n2", 1);
//    g.link_nodes("n1", "n22", 2); // won't do anything
//
//    println!();
//    println!(
//        "n1's data before being modified: {:?}",
//        g.get_node_data("n1")
//    );
//    {
//        // modify n1's data
//        let n1_data = g.get_mut_node_data("n1").unwrap();
//        (*n1_data).0 = 100;
//    }
//    println!("n1's data after modified: {:?}", g.get_node_data("n1"));
//    println!();
//
//    //    for n in g.iter_nodes() {
//    //        println!("node: {:?}", n)
//    //    }
//    //    println!();
//
//    println!("n1 outputs to: {:?}", g.get_outputs_for_node("n1"));
//    println!("n2 take input from: {:?}", g.get_inputs_for_node("n2"));
//}
