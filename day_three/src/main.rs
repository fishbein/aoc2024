use std::fs;

fn main() {
    let raw_input = fs::read_to_string("/Users/phil/Developer/aoc2024/day_three/src/input.txt")
        .expect("Should have been able to read the file");

    println!("{}", parse_instructions(&raw_input))
}

fn parse_instructions(instructions: &str) -> i32 {
    let mut num_one = String::from("");
    let mut num_two = String::from("");

    let mut current_position = 0;

    let mut result = 0;

    for c in instructions.chars() {
        if current_position == 0 && c == 'm'
            || current_position == 1 && c == 'u'
            || current_position == 2 && c == 'l'
            || current_position == 3 && c == '('
        {
            current_position += 1;
            continue;
        }

        if current_position == 4 {
            if c.is_digit(10) {
                num_one.push(c);
                continue;
            }
            if c == ',' {
                current_position += 1;
                continue;
            }
        }

        if current_position == 5 {
            if c.is_digit(10) {
                num_two.push(c);
                continue;
            }
            if c == ')' {
                let parsed_num_one: i32 = num_one.parse().expect("Failed to parse int from string");
                let parsed_num_two: i32 = num_two.parse().expect("Failed to parse int from string");
                result += parsed_num_one * parsed_num_two;
            }
        }

        num_one = String::from("");
        num_two = String::from("");
        current_position = 0;
    }

    return result;
}
