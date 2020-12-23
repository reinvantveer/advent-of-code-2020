use std::fs;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("input.txt");
    let adapters = lines_to_numbers(&lines);
    let chain = get_adapter_chain(&adapters);
    let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&chain);
    println!("There are {} 1-jolt diff adapters and {} 3-jolt diff adapters in the chain",
        &diffs_1_jolt, &diffs_3_jolt
    );
    println!("These numbers multiplied is {}", &diffs_1_jolt * &diffs_3_jolt);

    // Part 2
    let paths_count = iterate_contained_paths(&chain);

    println!("{} paths in total", paths_count);
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    input
        .lines()
        .map(String::from)
        .collect()
}

fn lines_to_numbers(lines: &Vec<String>) -> Vec<usize> {
    lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_adapter_chain(adapters: &Vec<usize>) -> Vec<usize> {
    // We need to include the outlet socket!
    let mut copied_chain = adapters.to_vec();
    copied_chain.sort();
    copied_chain.insert(0, 0);

    // Also: we need to include the built-in adapter
    let last_adapter = copied_chain.last().unwrap();
    let builtin_adapter = last_adapter + 3;
    copied_chain.push(builtin_adapter.to_owned());
    copied_chain
}

fn get_joltage_differences(adapter_chain: &Vec<usize>) -> (usize, usize) {
    let diffs: Vec<_> = adapter_chain[1..]
        .iter()
        .enumerate()
        .map(|(idx, adapter)| adapter - adapter_chain[idx])
        .collect();

    let diffs_1_jolt: Vec<_> = diffs
        .iter()
        .filter(|diff| **diff == 1 as usize)
        .collect();

    let diffs_3_jolt: Vec<_> = diffs
        .iter()
        .filter(|diff| **diff == 3 as usize)
        .collect();

    (diffs_1_jolt.len(), diffs_3_jolt.len())
}

fn make_adapter_graph(adapter_chain: &Vec<usize>) -> DiGraph<usize, usize> {
    let mut adapter_graph = DiGraph::<usize, usize>::new();

    for adapter in adapter_chain.iter() {
        adapter_graph.add_node(*adapter);
    }

    for adapter in adapter_chain {
        let compatible_adapters: Vec<_> = adapter_chain
            .iter()
            .filter(|maybe_compat_adapter| {
                let maybe_compat_adapter_isize = **maybe_compat_adapter as isize;
                let adapter_isize = *adapter as isize;
                let joltage_diff = maybe_compat_adapter_isize - adapter_isize;
                joltage_diff >=1 && joltage_diff <=3
            })
            .collect();

        let edges: Vec<_> = compatible_adapters
            .iter()
            .map(|compatible_adapter| {
                (get_node_idx(adapter, &adapter_graph).unwrap(),
                 get_node_idx(*compatible_adapter, &adapter_graph).unwrap(),
                 1)
            })
            .collect();

        adapter_graph.extend_with_edges(edges);

    }
    adapter_graph
}

fn get_node_idx(node_weight: &usize, graph: &DiGraph<usize, usize>) -> Option<NodeIndex<u32>> {
    let node_idx = graph
        .node_indices()
        .find(|i| graph[*i] == *node_weight);

    node_idx
}

fn count_contained_paths(graph: &DiGraph<usize, usize>, start_idx: &NodeIndex) -> usize {
    let mut path_count: usize = 0;

    // Get the outgoing edges that will lead to the device
    let outgoing_edges = graph
        .edges(*start_idx);

    // There's an extra path for each outgoing edge, minus one, but only if it connects to the device
    for (edge_idx, edge) in outgoing_edges.enumerate() {
        // Add to the count only if it actually adds an extra path
        if edge_idx > 0 { path_count += 1; }

        path_count += count_contained_paths(&graph, &edge.target());
    }

    path_count
}

fn iterate_contained_paths(adapters: &Vec<usize>) -> i128 {
    let jolts: Vec<_> = adapters
        .iter()
        .map(|adapter| *adapter as i32)
        .collect();
    // let jolt_range = -3_i32..*adapters.last().unwrap() as i32 + 3;

    // Map to range
    let mut paths: HashMap<i32, i128> = (-3_i32..*jolts.last().unwrap() as i32 + 3).map(|jolt| (jolt.clone(), 0)).collect();
    // The socket has jolt level 0, so there is 1 path to that.
    *paths.get_mut(&0).unwrap() = 1;

    // Each subsequent jolt-converter can be reached from the previous
    // two jolt levels, so add the paths that can reach that.
    for jolt in jolts {
        *paths.get_mut(&(jolt as i32)).unwrap() =
            paths[&(jolt as i32 - 3)] +
                paths[&(jolt as i32 - 2)] +
                paths[&(jolt as i32 - 1)];

        println!("Jolt: {}, Paths: {}", jolt, paths[&(jolt as i32)]);
    }

    let last_adapter = *adapters.last().unwrap();
    paths[&(last_adapter as i32)]
}

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, get_joltage_differences, get_adapter_chain, make_adapter_graph, get_node_idx, count_contained_paths, iterate_contained_paths};
    use petgraph::algo::all_simple_paths;

    #[test]
    fn test_adapter_chain() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);

        // Since the chain must include the outlet socket and the built-in adapter, it's longer
        assert_eq!(chain.len(), adapters.len() + 2);
        assert_eq!(chain, vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22])
    }

    #[test]
    fn test_jolt_differences() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);
        let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&chain);
        assert_eq!(diffs_1_jolt, 7);
        assert_eq!(diffs_3_jolt, 5);
    }

    #[test]
    fn test_slightly_less_simple_joltage_rating() {
        let lines = read_lines("example1_2.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);
        let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&chain);
        assert_eq!(diffs_1_jolt, 22);
        assert_eq!(diffs_3_jolt, 10);
    }

    // Part 2

    #[test]
    fn test_create_adapter_graph() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);
        let graph = make_adapter_graph(&chain);

        assert_eq!(graph.node_count(), adapters.len() + 2);
        assert_eq!(graph.edge_count(), 16);
    }

    #[test]
    fn test_num_paths() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);
        let graph = make_adapter_graph(&chain);

        let socket_idx = get_node_idx(chain.first().unwrap(), &graph).unwrap();
        let device_idx = get_node_idx(chain.last().unwrap(), &graph).unwrap();

        let paths: Vec<Vec<_>>
            = all_simple_paths(&graph, socket_idx, device_idx, 0, None)
                .collect();

        assert_eq!(paths.len(), 8);

        let paths_count = count_contained_paths(&graph, &socket_idx) + 1;
        assert_eq!(paths_count, 8);
    }

    #[test]
    fn test_bigger_num_paths() {
        let lines = read_lines("example1_2.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters);
        let graph = make_adapter_graph(&chain);

        let socket_idx = get_node_idx(chain.first().unwrap(), &graph).unwrap();
        let device_idx = get_node_idx(chain.last().unwrap(), &graph).unwrap();

        let paths: Vec<Vec<_>>
            = all_simple_paths(&graph, socket_idx, device_idx, 0, None)
                .collect();

        assert_eq!(paths.len(), 19208);

        let paths_count = count_contained_paths(&graph, &socket_idx) + 1;
        assert_eq!(paths_count, 19208);

        // The iterative method (contrasted with recursive method
        let paths_count = iterate_contained_paths(&chain);
        assert_eq!(paths_count, 19208);
    }
}