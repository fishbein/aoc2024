fn main() {
    let line_vectors: Vec<Vec<i32>> = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    println!(
        "{}",
        line_vectors
            .into_iter()
            .filter(|line| is_damp_safe(line))
            .count()
    );
}

fn is_safe(list: &[i32]) -> bool {
    (all_increasing(list) || all_decreasing(list)) && is_list_gradual(list)
}

fn is_damp_safe(list: &[i32]) -> bool {
    if is_safe(list) {
        return true;
    }

    for index in 0..list.len() {
        let mut list_copy = list.to_owned().clone();
        list_copy.remove(index);

        if is_safe(&list_copy) {
            return true;
        }
    }

    false
}

fn all_increasing(list: &[i32]) -> bool {
    for index in 1..list.len() {
        if list[index] > list[index - 1] {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn all_decreasing(list: &[i32]) -> bool {
    for index in 1..list.len() {
        if list[index] < list[index - 1] {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn is_list_gradual(list: &[i32]) -> bool {
    for index in 1..list.len() {
        let diff = (list[index] - list[index - 1]).abs();
        if (1..=3).contains(&diff) {
            continue;
        } else {
            return false;
        }
    }

    true
}
