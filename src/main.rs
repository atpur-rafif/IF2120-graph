#![allow(dead_code)]

mod dsu;
mod graph;

use crate::dsu::DSU;
use crate::graph::Graph;

fn main() {
    let graph = Graph::from_file("./src/graph/node.txt", "./src/graph/edge.txt");
    let mut dsu = DSU::from_graph(&graph).group();
    dsu.sort_by(|a, b| b.len().cmp(&a.len()));
    let sorted = dsu;

    for group in &sorted {
        println!(
            "{}\t{:?}",
            group.len(),
            group
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        );
    }
}
