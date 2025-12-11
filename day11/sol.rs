use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let outs : Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();

    let mut graph : HashMap<String, HashSet<&str>> = HashMap::new();
    for out in outs {
        let graph_input : Vec<&str> = out.split(" ").collect();
        let mut node_outs = HashSet::new();

        for i in 1..graph_input.len() {
            node_outs.insert(graph_input[i]);
        }
        let mut key = graph_input[0].to_string();
        key.pop();
        graph.insert(key, node_outs);
    }

    //println!("{:?}", graph);

    let paths_part1 = part1(&graph, "you");

    println!("number of paths: {paths_part1}");

    let paths_part2 = part2(&graph, "svr", false, false, HashSet::new());

    println!("number of dac and fft paths: {paths_part2}");
}

fn part2(
    graph : &HashMap<String, HashSet<&str>>, 
    start_node : &str, 
    dac : bool, 
    fft : bool, 
    mut passed : HashSet<String>) -> i32 
{
    if start_node == "out" {
        if dac && fft {
            return 1;
        } else {
            return 0;
        }
    } 

    if passed.contains(start_node) {
        return 0;
    }

    passed.insert(start_node.to_string());

    let outgoing = match graph.get(start_node) {
        Some(outs) => outs,
        None => return 0
    };

    let mut curr_dac = dac;
    let mut curr_fft = fft;

    if start_node == "dac" {
        curr_dac = true;
    } else if start_node == "fft" {
        curr_fft = true;
    }

    let mut node_paths = 0;
    for node in outgoing {
        node_paths += part2(graph, node, curr_dac, curr_fft, passed.clone());
    }

    return node_paths;
}

fn part1(graph : &HashMap<String, HashSet<&str>>, start_node : &str) -> i32 {
    if start_node == "out" {
        return 1;
    }
    
    let outgoing = match graph.get(start_node) {
        Some(outs) => outs, 
        None => {
            println!("not found {start_node}");
            return 0;
        }
    };

    let mut node_paths = 0;
    for node in outgoing {
        node_paths += part1(graph, node);
    }

    return node_paths;
}
