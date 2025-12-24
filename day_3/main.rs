use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn max_joltage(battery_bank_config: String) -> u64 {
    let bank_joltages: Vec<u64> = battery_bank_config.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut max_joltage: u64 = 0;
    for i in 0..(bank_joltages.len() - 1) {
        for j in (i+1)..bank_joltages.len() {
            if  10*bank_joltages[i] + bank_joltages[j] > max_joltage {
                max_joltage = 10*bank_joltages[i] + bank_joltages[j];
            }
        }
    }
    max_joltage
}


fn main() {
    let file_path = "./input.txt";
    let content_lines = read_lines(file_path);
    let joltages: Vec<u64> = content_lines.into_iter().map(max_joltage).collect();
    let joltage_sum: u64 = joltages.clone().into_iter().sum();
    println!("{:?} {}", joltages, joltage_sum);
}


