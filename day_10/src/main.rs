use std::fs;
use petgraph::graph::{DiGraph, NodeIndex};

fn main() {
    let lines = read_lines("input.txt");
    let adapters = lines_to_numbers(&lines);
    let chain = get_adapter_chain(&adapters);
    let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&chain);
    println!("There are {} 1-jolt diff adapters and {} 3-jolt diff adapters in the chain",
        &diffs_1_jolt, &diffs_3_jolt
    );
    println!("These numbers multiplied is {}", &diffs_1_jolt * &diffs_3_jolt);
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

fn make_adapter_graph(adapter_chain: &Vec<usize>) -> DiGraph<usize, ()> {
    let mut adapter_graph = DiGraph::<usize, ()>::new();

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

        for compatible_adapter in compatible_adapters {
            let adapter_idx = get_node_idx(adapter, &adapter_graph).unwrap();
            let compatible_idx = get_node_idx(compatible_adapter, &adapter_graph).unwrap();

            adapter_graph.add_edge(adapter_idx, compatible_idx, ());
        }
    }
    adapter_graph
}

fn get_node_idx(node_weight: &usize, graph: &DiGraph<usize, ()>) -> Option<NodeIndex<u32>> {
    let node_idx = graph
        .node_indices()
        .find(|i| graph[*i] == *node_weight);

    node_idx
}


#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, get_joltage_differences, get_adapter_chain, make_adapter_graph};
    use petgraph::graph::DiGraph;

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
        let graph: DiGraph<usize, ()> = make_adapter_graph(&chain);

        assert_eq!(graph.node_count(), adapters.len() + 2);
        assert_eq!(graph.edge_count(), 16);
    }
}