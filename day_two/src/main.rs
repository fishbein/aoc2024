use std::fs;

fn main() {
    let raw_input = fs::read_to_string("/Users/phil/Developer/aoc2024/day_two/src/input.txt")
        .expect("Should have been able to read the file");

    let lines = raw_input.lines();
    let mut line_vectors: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let split_line: Vec<i32> = line
            .split_whitespace()
            .map(|val| val.parse().expect("Failed to get number"))
            .collect();

        line_vectors.push(split_line);
    }

    println!(
        "{}",
        line_vectors
            .iter()
            .filter(
                |line| (all_increasing(&line) || all_decreasing(&line)) && is_list_gradual(&line)
            )
            .count()
    );
}

fn all_increasing(list: &Vec<i32>) -> bool {
    for index in 1..list.len() {
        if list[index] > list[index - 1] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn all_decreasing(list: &Vec<i32>) -> bool {
    for index in 1..list.len() {
        if list[index] < list[index - 1] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn is_list_gradual(list: &Vec<i32>) -> bool {
    for index in 1..list.len() {
        let diff = (list[index] - list[index - 1]).abs();
        if diff >= 1 && diff <= 3 {
            continue;
        } else {
            return false;
        }
    }
    return true;
}
