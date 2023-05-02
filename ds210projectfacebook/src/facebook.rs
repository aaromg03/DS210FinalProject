use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::dijkstra;
use std::collections::HashMap;

fn main() {
    let file = File::open("facebook_combined.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create an empty undirected graph
    let mut graph = UnGraph::<i32, i32>::new_undirected();

    


    // Add edges to the graph
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let source_idx = parts.next().unwrap().parse::<usize>().unwrap();
        let target_idx = parts.next().unwrap().parse::<usize>().unwrap();

      

        for i in 0..=std::cmp::max(source_idx, target_idx) {
            graph.add_node(i.try_into().unwrap());
        }
        
        


        let source = NodeIndex::new(source_idx);
        let target = NodeIndex::new(target_idx);
        graph.add_edge(source, target, 1);

        


        

        
    }

    let mut total_distance = 0;
    let mut num_pairs = 0;

    //Here I used Dijkstra's algorithm to find shortest path
    for i in 0..graph.node_count() {
        for j in 0..graph.node_count() {
            if i == j {
                continue;
            }

            let start = NodeIndex::new(i);
            let end = NodeIndex::new(j);

            let distances: Option<HashMap<NodeIndex, i32>> = Some(dijkstra(&graph, start, Some(end), |_| 1));
            if let Some(distances) = distances {
                if let Some(distance) = distances.get(&end) {
                    total_distance += *distance;
                    num_pairs += 1;
                } else {
                    println!("No path found between nodes {} and {}", i, j);
                }
            } else {
                println!("No path found between nodes {} and {}", i, j);
            }
        }
    }

    // Solving for the average distance
    let average_distance = if num_pairs == 0 {
        println!("No pairs of vertices have a path between them");
        0.0
    } else {
        total_distance as f64 / num_pairs as f64
    };

    println!("Average distance: {:.2}", average_distance);
}
