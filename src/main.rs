#![allow(dead_code)]

mod dsu;
mod graph;

use crate::dsu::DSU;
use crate::graph::Graph;

fn main() {
    let graph = Graph::from_file("./src/graph/node.txt", "./src/graph/edge.txt");
    let dsu = DSU::from_graph(&graph);

    println!("{:?}", dsu.group());
}
