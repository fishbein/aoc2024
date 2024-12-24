fn main() {
    let input = include_str!("./input.txt");

    let (list_one, list_two) = parse_and_sort_lists(input);

    let mut total_distance: i32 = 0;

    for index in 0..list_one.len() {
        total_distance += (list_one[index] - list_two[index]).abs();
    }

    println!("{total_distance}");

    let mut total_similarity_scores: i32 = list_one
        .iter()
        .map(|n| get_similarity_score(*n, &list_two))
        .sum::<i32>();

    for list_value in list_one {
        total_similarity_scores += get_similarity_score(list_value, &list_two)
    }
    println!("{total_similarity_scores}")
}

fn parse_and_sort_lists(list: &str) -> (Vec<i32>, Vec<i32>) {
    let pairs = list.lines();

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in pairs {
        let (num_one, num_two) = line.split_once("   ").expect("Failed to parse line");

        list_one.push(num_one.parse().expect("Failed to parse string"));
        list_two.push(num_two.parse().expect("Failed to parse string"));
    }

    list_one.sort_unstable();
    list_two.sort_unstable();

    (list_one, list_two)
}

fn get_similarity_score(value: i32, compare_list: &[i32]) -> i32 {
    let appearances: i32 = compare_list.iter().filter(|&val| *val == value).count() as i32;
    appearances * value
}
