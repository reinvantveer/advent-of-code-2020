use std::fs;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("input.txt");
    let group_answers = get_anyone_group_answers(lines.clone());
    let score = get_anyone_answers_score(group_answers);

    println!("Any answer score: {}", score);

    let everyone_group_answers = get_everyone_group_answers(lines);
    let everyone_score = everyone_answers_total_score(everyone_group_answers);

    println!("Everyone answers total score: {}", everyone_score);
}

fn everyone_answers_total_score(everyone_group_answers: Vec<usize>) -> usize {
    println!("Number of groups: {}", everyone_group_answers.len());

    let mut everyone_score: usize = 0;
    for score in everyone_group_answers {
        everyone_score += score;
    }
    everyone_score
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn get_anyone_group_answers(lines: Vec<String>) -> Vec<String> {
    let mut grouped: Vec<String> = Vec::new();
    let mut one_group: String = String::from("");

    for line in lines {
        if line == "" {
            grouped.push(one_group.to_owned());
            one_group = "".to_string();
        } else {
            // Keep adding to the group unless the item is already there
            for char in line.chars() {
                if !(one_group.contains(char)) {
                    one_group += &char.to_string();
                }
            }
        }
    }
    // add the last group
    grouped.push(one_group);
    grouped
}

fn get_everyone_group_answers(lines: Vec<String>) -> Vec<usize> {
    let mut group_scores: Vec<usize> = Vec::new();
    let mut one_group: HashMap<char, usize> = HashMap::new();
    let mut num_group_members = 0;

    for line in lines {
        if line == "" { // End of the group
            // Tally the score
            let mut score = 0;

            for &num_answers in one_group.values() {
                if num_answers == num_group_members {
                    score += 1;
                }
            }
            group_scores.push(score);

            // Clean up for next group
            one_group.drain();
            num_group_members = 0;
        } else {
            num_group_members += 1;

            for character in line.chars() {
                let counter = one_group.entry(character).or_insert(0);
                *counter += 1;
            }
        }
    }
    // Add the last group
    let mut score = 0;
    for &num_answers in one_group.values() {
        if num_answers == num_group_members {
            score += 1;
        }
    }
    group_scores.push(score.to_owned());

    group_scores
}

fn get_anyone_answers_score(grouped_answers: Vec<String>) -> usize {
    let mut score = 0 as usize;

    for answer in grouped_answers {
        score += answer.len();
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn figure_out_correct_example_answer() {
        let lines = read_lines("example.txt");
        let group_answers = get_anyone_group_answers(lines);
        assert_eq!(group_answers[0], "abc");
        assert_eq!(group_answers[1], "abc");
        assert_eq!(group_answers[2], "abc");
        assert_eq!(group_answers[3], "a");
        assert_eq!(group_answers[4], "b");
    }

    #[test]
    fn test_get_anyone_answer_score() {
        let lines = read_lines("example.txt");
        let group_answers = get_anyone_group_answers(lines);
        let score = get_anyone_answers_score(group_answers);
        assert_eq!(score, 11);
    }

    #[test]
    fn test_everyone_answer_score() {
        let lines = read_lines("example.txt");
        let everyone_group_answers = get_everyone_group_answers(lines);

        let expected= vec![3, 0, 1, 1, 1];
        assert_eq!(everyone_group_answers.len(), expected.len());
        assert_eq!(everyone_group_answers, expected);

        let total = everyone_answers_total_score(everyone_group_answers);
        assert_eq!(total, 6);
    }

    #[test]
    fn cheat() {
        let lines = fs::read_to_string("example.txt").unwrap();
        let score = solve2(&lines);
        assert_eq!(score, Some(6));
    }
}