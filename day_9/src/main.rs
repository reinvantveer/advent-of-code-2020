use std::fs;

fn main() {
    let lines = read_lines("input.txt");
    let numbers = lines_to_numbers(&lines);
    let first_wrong_number = find_first_wrong_number(numbers, 25);
    println!("First wrong number: {}", first_wrong_number);
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

fn find_first_wrong_number<'a>(
    numbers: Vec<usize>,
    preamble_len: usize
) -> usize {
    let verifiable_numbers = numbers[preamble_len..].iter();

    for (idx, num_to_check) in verifiable_numbers.enumerate() {
        // Preamble start = idx because at idx = 0: preamble_start = 0
        let preamble_stop = idx + preamble_len + 1;  // Cuz at idx = 0 preamble_stop = 6
        let preamble = &numbers[idx..preamble_stop];

        let cartesian = cartesian_product(&[&preamble, &preamble]);

        let sums: Vec<usize> = cartesian
            .iter()
            .filter(|combi| combi[0] != combi[1])
            .map(|combi| combi[0] + combi[1])
            .collect();

        if !sums.contains(num_to_check) {
            println!("{:?}", num_to_check);
            return *num_to_check;
        }
    }
    unreachable!("No errors found");
}

// Thanks kylewlacy: https://gist.github.com/kylewlacy/115965b40e02a3325558
fn cartesian_product<T: Clone>(lists: &[&[T]]) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter().cloned().fold(init, |vec, list| {
                partial_cartesian(vec, list)
            })
        },
        None => {
            vec![]
        }
    }
}

// Thanks kylewlacy: https://gist.github.com/kylewlacy/115965b40e02a3325558
fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
    a.into_iter().flat_map(|xs| {
        b.iter().cloned().map(|y| {
            let mut vec = xs.clone();
            vec.push(y);
            vec
        }).collect::<Vec<_>>()
    }).collect()
}

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, find_first_wrong_number};

    #[test]
    fn test_first_wrong_number() {
        let lines = read_lines("example.txt");
        let numbers = lines_to_numbers(&lines);
        let wrong = find_first_wrong_number(numbers, 5);
        assert_eq!(wrong, 127)
    }
}