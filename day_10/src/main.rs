use std::fs;
use std::convert::TryFrom;

fn main() {
    println!("Hello, world!");
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

fn get_device_rating(ratings: &Vec<usize>) -> usize {
    ratings.iter().max().unwrap() + 3
}

fn get_adapter_chain(adapters: &Vec<usize>, cur_rating: usize) -> Option<Vec<usize>> {
    // pluggable adapters have a rating 1-3 higher than the current one
    let pluggables: Vec<_> = adapters
        .iter()
        .enumerate()
        .map(|(idx, r)| (idx, isize::try_from(*r).unwrap()))
        .filter(|(_, r )| *r - cur_rating as isize >= 1 && *r - cur_rating as isize <= 3)
        .map(|v| v.to_owned())
        .collect();

    // We're at the last good adapter in the set if there's only one good candidate in the adapters
    if pluggables.len() == 1 && adapters.len() == 1 {
        return Some(adapters.to_vec())
    }

    // Brute-force try all possible paths by iterating over candidates from adapters that can plug
    // into the parent
    for (idx, candidate) in pluggables {
        // The remaining adapters are all adaptors except the one in the for loop
        let mut remaining_adapters = adapters.to_vec();
        remaining_adapters.remove(idx);  // Don't include the current adapter in the leftovers

        // Try to get the chain using the current remaining adapters
        // Except that the rating is now different: it's the candidate adapter in the loop
        let candidate_chain =
            get_adapter_chain(&remaining_adapters, candidate as usize);

        // If there is no valid path from the remaining adapters, using this current adapter
        // then this was not a good path and we continue with the next adapter

        if candidate_chain == None {
            continue;
        }

        // Unpack the result from analyzing the remaining adapters
        let returned_chain = candidate_chain.unwrap();

        // The path is good: it includes all elements
        if returned_chain.len() == adapters.len() - 1 {
            let mut complete_chain = vec![candidate as usize];
            complete_chain.extend(returned_chain);

            // If we return full circle, having searched all paths
            if cur_rating == 0 {
                // Now we only have to add the built-in device adapter
                let last_adapter = complete_chain.last().unwrap();
                let builtin_adapter = last_adapter + 3;
                complete_chain.push(builtin_adapter.to_owned());
            }

            return Some(complete_chain)
        }
    }

    // If none of the paths in the for loop resulted in a valid chain of adapters, then return
    // None
    None
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

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, get_device_rating, get_joltage_differences, get_adapter_chain};

    #[test]
    fn test_simple_device_joltage_rating() {
        let lines = read_lines("example1_1.txt");
        let ratings = lines_to_numbers(&lines);
        let device_rating = get_device_rating(&ratings);

        assert_eq!(device_rating, 22);
    }

    #[test]
    fn test_adapter_chain() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters, 0);
        let good_chain = chain.unwrap();

        assert_eq!(good_chain.len(), adapters.len() + 1);
        assert_eq!(good_chain, vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22])
    }

    #[test]
    fn test_jolt_differences() {
        let lines = read_lines("example1_1.txt");
        let adapters = lines_to_numbers(&lines);
        let chain = get_adapter_chain(&adapters, 0);
        let good_chain = chain.unwrap();
        let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&good_chain);
        assert_eq!(diffs_1_jolt, 7);
        assert_eq!(diffs_3_jolt, 5);
    }

    #[test]
    fn test_slightly_less_simple_joltage_rating() {
        let lines = read_lines("example1_2.txt");
        let ratings = lines_to_numbers(&lines);
        let device_rating = get_device_rating(&ratings);

        assert_eq!(device_rating, 22);
    }


}