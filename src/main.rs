mod graph;

use crate::graph::Graph;

fn main() {
    let graph = Graph::from_file("./src/graph/node.txt", "./src/graph/edge.txt");

    println!("{:?}", graph);
}
