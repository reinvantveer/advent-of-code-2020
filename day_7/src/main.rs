extern crate petgraph;

use std::fs;
use std::collections::HashMap;
use petgraph::algo::{has_path_connecting};
use petgraph::graph::{DiGraph};

struct BagRule {
    bag_type: String,
    contains: HashMap<String, usize>
}

fn main() {
    let lines = read_lines("input.txt");
    let rules = parse_graph(lines);
    let color_to_look_for = "shiny gold".to_string();
    let num_colors = num_bags_that_contain(color_to_look_for.clone(), rules);
    println!("{} bags can hold {}", num_colors, color_to_look_for);
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn parse_graph(lines: Vec<String>) -> DiGraph<String, ()> {
    let mut rules_graph = DiGraph::new();
    for line in lines {
        let parts = parse_subgraph_parts(line);

        let has_node = rules_graph.node_indices().find(|i| rules_graph[*i] == parts.bag_type);
        if has_node == None {
            rules_graph.add_node(parts.bag_type.to_string());
        }

        for sub_bag_type in parts.contains.keys() {
            // Add the sub-bag type if it doesn't exist yet
            let has_node = rules_graph.node_indices().find(|i| rules_graph[*i] == *sub_bag_type);
            if has_node == None {
                rules_graph.add_node(sub_bag_type.to_string());
            }

            // Add the edge between the bag and the sub-bag
            let source = rules_graph
                .node_indices()
                .find(|i| rules_graph[*i] == parts.bag_type)
                .unwrap();

            let target = rules_graph
                .node_indices()
                .find(|i| rules_graph[*i] == *sub_bag_type)
                .unwrap();

            rules_graph.add_edge(source, target, ());
        }
    }
    rules_graph
}

fn parse_subgraph_parts(line: String) -> BagRule {
    let line_split: Vec<&str> = line.split(" bags contain ").collect();
    let bag_type = line_split[0];

    let mut rule = BagRule {
        bag_type: (*bag_type.to_string()).parse().unwrap(),
        contains: Default::default()
    };

    match line_split[1].contains("no other bags") {
        true => (()),
        false => extract_subbag_rules(line_split, &mut rule)
    };

    rule
}

fn extract_subbag_rules(line_split: Vec<&str>, rule: &mut BagRule) {
    let contained: Vec<String> = line_split[1]
        .strip_suffix(".").unwrap()// Get rid of the last dot in the phrase
        .replace(" bags", "")// Get rid of all spurious "bags" references
        .replace(" bag", "")// Get rid of all spurious "bag" references next
        .split(", ")// Split whatever sub bags are within by comma-separated
        .map(|sub_bag| sub_bag.to_string())
        .collect();

    for num_and_sub_bag in contained {
        let mut subparts = num_and_sub_bag.split(" ");
        let num = subparts.next().unwrap().parse().unwrap();
        let sub_bag_type = subparts
            .map(|part| part.to_string())
            .collect::<Vec<String>>().join(" ");

        rule.contains.insert(sub_bag_type, num);
    }
}

fn num_bags_that_contain(color: String, rules: DiGraph<String, ()>) -> usize {
    let target_node =  rules
        .node_indices()
        .find(|i| rules[*i] == color)
        .unwrap();

    for source_node in rules.node_indices() {
        if target_node == source_node { continue; }
        if has_path_connecting(&rules, source_node, target_node, None) {
            println!("{:?} connects to {:?}", &source_node, &target_node);
        }
    }

    let num_paths: Vec<_>= rules
        .node_indices()
        .filter(|source_node| *source_node != target_node)
        .filter(|source_node| has_path_connecting(&rules, *source_node, target_node, None))
        .collect();

    num_paths.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parser() {
        let lines = read_lines("example.txt");
        let parts1 = parse_subgraph_parts(lines[0].clone());

        assert_eq!(parts1.bag_type, "light red".to_string());
        assert_eq!(*parts1.contains.get("bright white").unwrap(), 1 as usize);
        assert_eq!(*parts1.contains.get("muted yellow").unwrap(), 2 as usize);

        let last_rule_set = parse_subgraph_parts(lines.last().unwrap().clone());
        assert_eq!(last_rule_set.bag_type, "dotted black".to_string());
    }

    #[test]
    fn test_graph_builder() {
        let lines = read_lines("example.txt");
        let rules = parse_graph(lines);

        let expected_nodes = vec![
            "light red",
            "bright white",
            "muted yellow",
            "dark orange",
            "shiny gold",
            "faded blue",
            "dark olive",
            "vibrant plum",
            "dotted black",
        ];

        assert_eq!(rules.node_count(), expected_nodes.len());
        assert_eq!(rules.edge_count(), 13);
    }

    #[test]
    fn test_correct_sample_answer() {
        let lines = read_lines("example.txt");
        let rules: DiGraph<String, ()> = parse_graph(lines);
        let num_bags = num_bags_that_contain("shiny gold".to_string(), rules);
        assert_eq!(num_bags, 4);
    }
}
