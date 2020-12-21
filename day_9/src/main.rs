use std::fs;

fn main() {
    let lines = read_lines("input.txt");
    let numbers = lines_to_numbers(&lines);
    let first_wrong_number = find_first_wrong_number(numbers.to_vec(), 25);
    println!("First wrong number: {}", &first_wrong_number);

    let (start, end) = get_contiguous_sum_range(&numbers, first_wrong_number);
    println!("Start: {}, end: {}", &start, &end);
    let sum_numbers = &numbers[start..end + 1];
    println!("Numbers in this range: {:?}", &sum_numbers);

    let min_val = sum_numbers.iter().min().unwrap();
    let max_val = sum_numbers.iter().max().unwrap();
    println!("Min value: {}", min_val);
    println!("Min value: {}", max_val);
    println!("Sum of min and max: {}", min_val + max_val);
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

fn find_first_wrong_number(
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

fn get_contiguous_sum_range(numbers: &Vec<usize>, sum: usize) -> (usize, usize) {
    // First loop: for the start index
    for first_idx in 0..numbers.len() {
        let mut last_idx = first_idx + 1;
        let mut slice_sum = numbers[first_idx];

        // Second loop: to detect the end of the sum range
        while slice_sum < sum && last_idx < numbers.len(){
            slice_sum += numbers[last_idx];

            if slice_sum == sum {  // The sum was found!
                println!("Sum {} found: starting at {}, ending at {}",
                         &slice_sum, &first_idx, &last_idx);
                return (first_idx, last_idx)
            }
            last_idx += 1;
        }
        println!("Start index {} did not yield a contiguous sum", first_idx);
    }
    (0, 0)
}

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, find_first_wrong_number, get_contiguous_sum_range};

    #[test]
    fn test_first_wrong_number() {
        let lines = read_lines("example.txt");
        let numbers = lines_to_numbers(&lines);
        let wrong = find_first_wrong_number(numbers, 5);
        assert_eq!(wrong, 127)
    }

    #[test]
    fn test_contiguous_sum() {
        let lines = read_lines("example.txt");
        let numbers = lines_to_numbers(&lines);
        let (first_idx, last_idx) = get_contiguous_sum_range(&numbers, 127);
        assert_eq!(first_idx, 2);
        assert_eq!(last_idx, 5)
    }
}