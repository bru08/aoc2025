use std::fs;

fn remove_multispaces(text: String) -> String {
    let mut space_last: bool = false;
    let mut result: Vec<char> = Vec::new();
    for string_char in text.chars() {
        if string_char != ' ' {
            result.push(string_char);
            space_last = false;
        } else if string_char == ' ' && !space_last{
            result.push(string_char);
            space_last = true;
        }
    }
    result.into_iter().collect::<String>()
}

fn trim_spaces(text: String) -> String {
    let result: Vec<char> = text.chars().collect();
    let (mut lower, mut upper): (usize, usize) = (0, result.len() - 1);
    let (mut lower_ok, mut upper_ok): (bool, bool) = (false, false);
    while lower<upper && !(lower_ok && upper_ok) {
        if result[lower] != ' ' {
            lower_ok = true;
        } else {
            lower += 1;
        }

        if result[upper] != ' ' {
            upper_ok = true;
        } else {
            upper -= 1;
        }
    }
    result[lower..=upper].into_iter().collect::<String>()
}

fn part1_total(text_input: &String) -> u64 {
    let text_lines: Vec<String> = text_input.clone()
                                        .lines()
                                        .map(String::from)
                                        .map(remove_multispaces)
                                        .map(trim_spaces)
                                        .collect();
    let input_data: Vec<Vec<u64>> = text_lines[0..(text_lines.len()-1)].into_iter().map(|x| x.split(' ').map(|y| y.parse::<u64>().unwrap()).collect()).collect();
    let operators: Vec<char> = text_lines.last().unwrap().split(' ').map(|s| s.chars().next().unwrap()).collect();
    let (rows, cols): (usize, usize) = (input_data.len(), input_data[0].len());
    let mut results: Vec<u64> = Vec::new();
    for j in 0..cols {
        let mut op_data: Vec<u64> = Vec::new();
        for i in 0..rows {
            op_data.push(input_data[i][j]);
        }
        if operators[j] == '+' {
            results.push(op_data.iter().sum());
        }else if operators[j] == '*'{
            results.push(op_data.iter().product());
        }

    }
    results.iter().sum()
}




fn main() {
    let file_path = "./input.txt";
    let text_input: String = fs::read_to_string(file_path).unwrap();
    let sum_results_part1: u64 = part1_total(&text_input);
    println!("[Part 1] Sum of all results in the sheet: {sum_results_part1}");
    let input_lines: Vec<&str> = text_input.lines().collect();
    let input_lines_char: Vec<Vec<char>> = input_lines.iter().map(|x| x.chars().collect()).collect();
    // following problem statement, naturally iterate right to left and bottom to top for digit relevance.
    let mut results: Vec<u64> = Vec::new();
    let mut operation_elements: Vec<u64> = Vec::new();
    let mut closing_operation: bool = false;
    for col_id in (0..input_lines_char[0].len()).rev() {
        if closing_operation {
            closing_operation = false;
            continue;
        }
        let mut digits: Vec<u64> = Vec::new();
        for row_id in 0..input_lines_char.len()-1 {
            // collect operation elment digits
            if input_lines_char[row_id][col_id] != ' ' {
                digits.push(input_lines_char[row_id][col_id].to_digit(10).unwrap() as u64);
            }
        }
        // Compose element number
        let mut element: u64 = 0;
        for (i, digit) in digits.iter().enumerate(){
            element += digit * 10_u64.pow((digits.len() -i -1) as u32);
        }
        // println!("Adding element {}", element);
        operation_elements.push(element);
        //
        if input_lines_char[input_lines_char.len()-1][col_id] == '+' {
            results.push(operation_elements.iter().sum());
            // println!("Operation elements: {:?}", operation_elements);
            operation_elements.clear();
            closing_operation=true;
        } else if input_lines_char[input_lines_char.len()-1][col_id] == '*' {
            results.push(operation_elements.iter().product());
            // println!("Operation elements: {:?}", operation_elements);
            operation_elements.clear();
            closing_operation=true;
        }
    }

    
    //let input_data: &[String] = &text_lines[..text_lines.len()-1];
    println!("[Part 2] Sum of all results in the sheet: {:?}", results.iter().sum::<u64>())
}