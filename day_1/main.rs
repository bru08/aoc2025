use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn parse_dial_instruction(instruction: &String) -> i32 {
    let mut delta: i32 = 0;
    if instruction.chars().next().unwrap() == 'L' {
        delta = instruction[1 .. instruction.len()].parse().unwrap();  // here text should be utf8 encoded, and each character should take only 1 byte
        delta = -1 * delta;
    } else if instruction.chars().next().unwrap() == 'R' {
        delta = instruction[1 .. instruction.len()].parse().unwrap();
    }
    delta
    
}

/*
fn update_dial(current_counter: &i32, delta: i32, max_val: i32) -> i32 {
    // Handle updates on a constrained range [0, 99]
    let delta_regularised = delta % max_val;
    let mut result = current_counter + delta_regularised;
    if result >= 0 {
        result = result % max_val;
    } else if result < 0 {
        result = max_val + result;
    }
    result
}
*/

fn update_dial_with_zero_crossings2(current_counter: &i32, delta: i32, max_val: i32) -> (i32, u32) {
    /*
    Update dial (circular array) counting each time the zero position is ecountered during a movement.
    */
    let mut result = current_counter + delta;
    let mut zero_crossings = (result / max_val).abs() as u32;
    result = result % max_val;
    if result < 0 {
        result = max_val + result;
        if *current_counter > 0 {
            zero_crossings += 1;
        }
    } else if delta < 0 && result == 0 && *current_counter > 0 {
        // count zero passages when updating towards left
        zero_crossings += 1;
    }
    (result, zero_crossings)
}

fn main() {
    let file_path = "./input.txt";
    let content_lines = read_lines(file_path);
    let mut dial : i32 = 50;
    let mut zero_crossing : u32 = 0;
    let mut zero_crossing_v2: u32 = 0;
    let mut additional_zero_crossings: u32;
    for instruction in &content_lines {
        (dial, additional_zero_crossings) = update_dial_with_zero_crossings2(&dial, parse_dial_instruction(instruction), 100);
        zero_crossing_v2 += additional_zero_crossings;
        if dial == 0 {
            zero_crossing += 1;
        }
    }
    println!("Final dial position {}", dial);
    println!("Total dial zero crossings {}", zero_crossing);
    println!("Total dial zero crossings with in-update crossings {}", zero_crossing_v2);
}