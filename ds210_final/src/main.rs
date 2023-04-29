mod graph;
mod plot;

use graph::Graph;
use plot::plot_degree_distribution;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read in the graph data from file
    let file = File::open("twitter_combined.txt").unwrap();
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let ids: Vec<&str> = line.split_whitespace().collect();
        let source = ids[0].parse::<usize>().unwrap();
        let target = ids[1].parse::<usize>().unwrap();
        graph.add_edge(source, target);
    }

    // Calculate and plot the degree distribution of the graph
    match plot_degree_distribution(&graph.nodes().iter().map(|&node| graph.degree(node)).collect::<Vec<_>>()) {
        Ok(()) => println!("Successfully plotted degree distribution"),
        Err(e) => eprintln!("Error plotting degree distribution: {}", e),
    }

    // Find all nodes that are neighbors of neighbors of node 0
    let node_0_neighbors_2 = graph.neighbors_at_distance_2(0);
    println!("Neighbors at distance 2 of node 0: {:?}", node_0_neighbors_2);
}
