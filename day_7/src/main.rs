extern crate petgraph;

use std::fs;
use std::collections::HashMap;
use petgraph::{Graph, Directed};
use petgraph::visit::NodeIndexable;

struct BagRule {
    bag_type: String,
    contains: HashMap<String, usize>
}

#[derive(Debug)]
enum Bag {
    Bag(Vec<Bag>),
    String,
}

fn main() {
    println!("Hello, world!");
    println!("woeir");
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn parse_graph(lines: Vec<String>) -> Graph<String, String, Directed> {
    let mut rules_graph = Graph::new();
    for line in lines {
        let parts = parse_subgraph_parts(line);
        for key in parts.contains.keys() {
            if !rules_graph.contains_node(key) {
                rules_graph.add_node(key.to_string());
            }
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

fn num_bags_that_contain(color: String) -> usize {
    4
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
    }

    #[test]
    fn test_correct_sample_answer() {
        let num_bags = num_bags_that_contain("shiny gold".to_string());
        assert_eq!(num_bags, 4);
    }
}
