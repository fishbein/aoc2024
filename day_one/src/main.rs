use std::fs;
fn main() {
    let raw_input = fs::read_to_string("/Users/phil/Developer/aoc2024/day_one/src/input.txt")
        .expect("Should have been able to read the file");

    let [list_one, list_two] = parse_and_sort_lists(&raw_input);

    let mut total_distance: i32 = 0;

    for index in 0..list_one.len() {
        total_distance += (list_one[index] - list_two[index]).abs();
    }

    println!("{total_distance}");

    let mut total_similarity_scores: i32 = 0;
    for list_value in list_one {
        total_similarity_scores += get_similarity_score(list_value, &list_two)
    }
    println!("{total_similarity_scores}")
}

fn parse_and_sort_lists(list: &str) -> [Vec<i32>; 2] {
    let pairs = list.lines();

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in pairs {
        let mut split_line = line.split_whitespace();
        let num_one = split_line.next().expect("No num one!");
        let num_two = split_line.next().expect("No num two!");

        list_one.push(num_one.parse().expect("Failed to parse string"));
        list_two.push(num_two.parse().expect("Failed to parse string"));
    }

    list_one.sort();
    list_two.sort();

    return [list_one, list_two];
}

fn get_similarity_score(value: i32, compare_list: &Vec<i32>) -> i32 {
    let appearances: i32 = compare_list.iter().filter(|&val| *val == value).count() as i32;
    return appearances * value;
}
