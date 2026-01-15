use std::fs;
use std::collections::{HashMap, HashSet};

type ConnectionMap = HashMap<String, Vec<String>>;
type CacheKey = (String, Vec<String>);
type Cache = HashMap<CacheKey, u64>;

fn count_paths(start_node: &str, exit_node:&str, connection_map: &ConnectionMap) -> u64 {
    let mut counter: u64 = 0;
    for next_node in connection_map[start_node].iter() {
        if next_node == start_node {
            return 0
        } else if next_node == exit_node {
            return 1
        }
        counter += count_paths(next_node, exit_node, &connection_map);
    }
    counter
}

fn count_inspect_paths(start_node: &String, exit_node:&String, lookup_set: &HashSet<String>, visited: &HashSet<String>, connection_map: &ConnectionMap, cache: &mut Cache) -> u64 { 
    /*
    This function still counts paths, and only counts the ones that in the end have passed by all the nodes in lookup_set.
    As the graph is a DAG, we can use memoization by just using the current node, and how many node in the lookup set we have already visited.
    This allow for more cache hits and the algorithm terminates in a reasonbale amount of time.
    */   
    if visited.contains(start_node) {
        return 0;
    }
    let mut visited_vec: Vec<String> = visited.iter()
                                              .filter(|x| lookup_set.contains(*x))
                                              .cloned()
                                              .collect();
    visited_vec.sort();
    let key = (start_node.clone(), visited_vec);
    if let Some(v) = cache.get(&key) {

        return *v;
    }

    let mut visited = visited.clone();
    visited.insert(start_node.to_string());
    //println!("Visiting {} visited len {}", start_node, visited.len());
    if start_node == exit_node {
        if lookup_set.is_subset(&visited) {
            return 1
        } else {
            return 0
        }
    }
    let mut counter: u64 = 0;
    for next_node in connection_map[start_node].iter() {
        counter += count_inspect_paths(next_node, exit_node, &lookup_set, &visited, connection_map, cache);
    }
    cache.insert(key, counter);
    counter
}

fn main() {
    let input_text: Vec<String> = fs::read_to_string("./input.txt")
                                .unwrap()
                                .lines()
                                .map(String::from)
                                .collect();
    let input_map: ConnectionMap = input_text.iter()
                                                .map(|x| {
                                                    let (k, v) = x.split_once(":").unwrap();
                                                    let v_split: Vec<String> = v.split_whitespace().map(String::from).collect();
                                                    (k.to_string(),v_split)
                                                }).collect();

    let n_paths_part_1: u64 = count_paths("you", "out", &input_map);
    println!("[Part1] n paths: {n_paths_part_1}");
    //
    let visited = HashSet::new();
    let lookup_set = HashSet::from(["dac".to_string(), "fft".to_string()]);
    let mut cache: Cache = HashMap::new();
    let res: u64 = count_inspect_paths(&"svr".to_string(), &"out".to_string(), &lookup_set, &visited, &input_map, &mut cache);
    println!("{}", res);
}
