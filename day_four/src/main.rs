fn main() {
    let input = include_str!("./input.txt").lines();

    let input_lines: Vec<String> = input.map(String::from).collect();

    part_one(&input_lines);
    part_two(&input_lines);
}

fn part_one(input_lines: &[String]) {
    let mut xmas_count = 0;

    for (line_index, line) in input_lines.iter().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            if c == 'X' {
                xmas_count += [
                    check_left(char_index, line),
                    check_right(char_index, line),
                    check_up(line_index, char_index, input_lines),
                    check_down(line_index, char_index, input_lines),
                    check_up_left(line_index, char_index, input_lines),
                    check_up_right(line_index, char_index, input_lines),
                    check_down_left(line_index, char_index, input_lines),
                    check_down_right(line_index, char_index, input_lines),
                ]
                .iter()
                .filter(|val| **val)
                .count();
            }
        }
    }
    println!("XMAS Count: {xmas_count}")
}

fn part_two(input_lines: &[String]) {
    let mut x_mas_count = 0;

    for (line_index, line) in input_lines.iter().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            if c == 'A' {
                let up_left_char = if line_index > 0 && char_index > 0 {
                    char_at_index(input_lines, line_index - 1, char_index - 1)
                } else {
                    ' '
                };
                let up_right_char = if line_index > 0 && char_index < input_lines[line_index].len()
                {
                    char_at_index(input_lines, line_index - 1, char_index + 1)
                } else {
                    ' '
                };
                let down_right_char = if line_index < input_lines.len()
                    && char_index < input_lines[line_index].len()
                {
                    char_at_index(input_lines, line_index + 1, char_index + 1)
                } else {
                    ' '
                };
                let down_left_char = if line_index < input_lines.len() && char_index > 0 {
                    char_at_index(input_lines, line_index + 1, char_index - 1)
                } else {
                    ' '
                };

                if ((up_left_char == 'M' && down_right_char == 'S')
                    || (up_left_char == 'S' && down_right_char == 'M'))
                    && ((down_left_char == 'M' && up_right_char == 'S')
                        || (down_left_char == 'S' && up_right_char == 'M'))
                {
                    x_mas_count += 1;
                }
            }
        }
    }
    println!("X-MAS count: {x_mas_count}")
}

fn check_left(char_index: usize, line: &str) -> bool {
    if char_index >= 3 {
        let line_vec = vec![
            char_at_index(&[line.to_string()], 0, char_index - 1),
            char_at_index(&[line.to_string()], 0, char_index - 2),
            char_at_index(&[line.to_string()], 0, char_index - 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check left: {line_string},");

        return line_string == "MAS";
    }

    false
}

fn check_right(char_index: usize, line: &str) -> bool {
    if char_index + 3 < line.len() {
        let line_vec = vec![
            char_at_index(&[line.to_string()], 0, char_index + 1),
            char_at_index(&[line.to_string()], 0, char_index + 2),
            char_at_index(&[line.to_string()], 0, char_index + 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check right: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_up(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index >= 3 {
        let line_vec = vec![
            char_at_index(lines, line_index - 1, char_index),
            char_at_index(lines, line_index - 2, char_index),
            char_at_index(lines, line_index - 3, char_index),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check up: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_down(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index + 3 < lines.len() {
        let line_vec = vec![
            char_at_index(lines, line_index + 1, char_index),
            char_at_index(lines, line_index + 2, char_index),
            char_at_index(lines, line_index + 3, char_index),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check down: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_down_right(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index + 3 < lines.len() && char_index + 3 < lines[0].len() {
        let line_vec = vec![
            char_at_index(lines, line_index + 1, char_index + 1),
            char_at_index(lines, line_index + 2, char_index + 2),
            char_at_index(lines, line_index + 3, char_index + 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check down right: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_down_left(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index + 3 < lines.len() && char_index >= 3 {
        let line_vec = vec![
            char_at_index(lines, line_index + 1, char_index - 1),
            char_at_index(lines, line_index + 2, char_index - 2),
            char_at_index(lines, line_index + 3, char_index - 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check down left: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_up_right(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index >= 3 && char_index + 3 < lines[0].len() {
        let line_vec = vec![
            char_at_index(lines, line_index - 1, char_index + 1),
            char_at_index(lines, line_index - 2, char_index + 2),
            char_at_index(lines, line_index - 3, char_index + 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check up right: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn check_up_left(line_index: usize, char_index: usize, lines: &[String]) -> bool {
    if line_index >= 3 && char_index >= 3 {
        let line_vec = vec![
            char_at_index(lines, line_index - 1, char_index - 1),
            char_at_index(lines, line_index - 2, char_index - 2),
            char_at_index(lines, line_index - 3, char_index - 3),
        ];
        let line_string: String = line_vec.into_iter().collect();

        println!("check up left: {line_string}");

        return line_string == "MAS";
    }

    false
}

fn char_at_index(lines: &[String], line_index: usize, char_index: usize) -> char {
    if line_index > lines.len() {
        return ' ';
    }

    let character = lines
        .get(line_index)
        .unwrap_or(&String::new())
        .chars()
        .nth(char_index)
        .unwrap_or(' ');

    // println!("Character at {line_index}, {char_index} = {character}");

    character
}
