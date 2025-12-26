use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn max_joltage(battery_bank_config: String) -> u64 {
    // First naive function to get two digits that represent max sum over the avialable battery bank.
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

fn max_joltage_parametrized(battery_bank_config: String, usable_batteries: u32) -> u64 {
    /* Greedy function that select the <usable_batteries> that get the highest final number.
    Its greedy as starting from the most relevant digit, in each round, it search in the window of available positions
    (battery_length - usable_batteries + already_selected_batteries + 1).

    Example:
    battery_config: 132451 usable batteries: 3
    round 1) search in subarray [1,3,2,4] -> select 4
    round 2) search in subarray [5,1] -> select 5
    round 3) search in subarray [1] -> select 1
    */
    if battery_bank_config.len() <= usable_batteries as usize {
        return battery_bank_config.parse::<u64>().unwrap()
    }
    let bank_joltages: Vec<u64> = battery_bank_config.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut joltage: u64 = 0;
    let mut last_index: usize = 0;
    let mut max_temp: u64 = 0;
    for i in 1..(usable_batteries+1) {

        for j in (last_index)..(battery_bank_config.len()-(usable_batteries-i) as usize){
            if bank_joltages[j] > max_temp {
                last_index = j;
                max_temp = bank_joltages[j];
            }
        }
        joltage += bank_joltages[last_index] * 10_u64.pow(usable_batteries-i);
        max_temp = 0;
        last_index += 1;
    }
    joltage
}

fn main() {
    let file_path = "./input.txt";
    let content_lines = read_lines(file_path);
    let joltages: Vec<u64> = content_lines.clone().into_iter().map(max_joltage).collect();
    let joltages_v2: Vec<u64> = content_lines.clone().into_iter().map(|x| max_joltage_parametrized(x, 12)).collect();
    let joltage_sum: u64 = joltages.clone().into_iter().sum();
    let joltage_sum_v2: u64 = joltages_v2.clone().into_iter().sum();
    println!("[Part1] Max joltage {}", joltage_sum);
    println!("[Part2] Max joltage: {}", joltage_sum_v2);
}


