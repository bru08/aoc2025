/*
Inspired by https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
*/

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

type DoubleHashMap = HashMap<Vec<i64>, HashMap<Vec<i64>, u64>>;


#[derive(Debug, Clone)]
struct LightSchema {
    lights: Vec<u8>,
    buttons: Vec<Vec<usize>>,
    power_requirements: Vec<u64>
}

fn read_input_day_10(filename: &str) -> Vec<LightSchema> {
    let lights: Vec<Vec<String>> = fs::read_to_string(filename)
                                .unwrap()
                                .lines()
                                .map(String::from)
                                .map(|x| x.split_whitespace().map(String::from).collect::<Vec<String>>())
                                .collect();
    let mut res: Vec<LightSchema> = Vec::new();
    for light_input in lights.iter() {
        let mut light: Vec<u8> = Vec::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut power_requirements: Vec<u64> = Vec::new();
        for item in light_input.iter() {
            if item.starts_with('[') {
                let content = &item[1..item.len()-1];
                light = content.chars().map(|x| if x=='.' {0_u8} else {1_u8}).collect();
            } else if item.starts_with('(') {
                let content = &item[1..item.len()-1];
                buttons.push(content.split(",").map(|x| x.parse::<usize>().unwrap()).collect());
            } else if item.starts_with('{') {
                let content = &item[1..item.len()-1];
                power_requirements = content.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
            } else {
                panic!("Unrecognised token in element {}", item)
            }
        }
        res.push(LightSchema{lights:light, buttons:buttons, power_requirements:power_requirements});
    }
    res
}

fn generate_possible_patterns(light_schema: &LightSchema) -> DoubleHashMap {
    // return  <parity_scheme>:[<pattern>:<# buttons>]
    // prepare empty nested dictionary
    let mut patterns_out: DoubleHashMap = HashMap::new();
    // iterate over all possible number of total button pressed
    for n_button_pressed in 0..=light_schema.buttons.len() {
        // given a total number of button pressed, and a set of button, generate all possbile button combinations
        let buttons_press_scheme_list: Vec<Vec<i32>>= (0..(light_schema.buttons.len() as i32)).combinations(n_button_pressed).collect();
        for buttons_press_scheme in buttons_press_scheme_list.iter() {
            let mut pattern: Vec<i64> = vec![0; light_schema.power_requirements.len()];
            for &button_id in buttons_press_scheme {
                // give a button pressing scheme, iterave over button and add to relevant counter
                for &counter_id in light_schema.buttons[button_id as usize].iter() {
                    pattern[counter_id] += 1;
                }
            }
            // deduce equivalent parity patterns (like the lights one)
            let parity_pattern: Vec<i64> = pattern.iter().map(|elem| elem%2).collect();
            // store for each pattern (total powr requirement) and its equivalent parity scheme with thei number of button pressed
            patterns_out.entry(parity_pattern.clone())
                        .or_insert_with(HashMap::new)
                        .entry(pattern.clone())
                        .and_modify(|v| *v = (*v).min(n_button_pressed as u64))
                        .or_insert(n_button_pressed as u64);
        }
    }
    patterns_out
}

fn solve_power_requirement(machine_config: &LightSchema) -> u64 {
    let patterns: DoubleHashMap = generate_possible_patterns(machine_config);
    fn solve_power_requirements_rec(power_req: &Vec<u64>, patterns: &DoubleHashMap) -> u64 {
        if power_req.iter().all(|&i| i==0) {return 0};
        let mut best_button_count = 1_000_000;
        let parity_pattern: Vec<i64> = power_req.iter().map(|elem| (elem%2) as i64).collect();
        if let Some(candidates) = patterns.get(&parity_pattern) {
            for (pattern, button_cost) in candidates {
                if pattern.iter().zip(power_req.iter()).all(|(&pattern_i, &power_req_i)| pattern_i <= power_req_i as i64) {
                    let new_power_req: Vec<u64> = pattern.iter()
                                                        .zip(power_req.iter())
                                                        .map(|(&pattern_i, &power_req_i)| (power_req_i - pattern_i as u64)/2)
                                                        .collect();
                    best_button_count = best_button_count.min(button_cost + 2 * solve_power_requirements_rec(&new_power_req, &patterns))
                }
                //println!("{:?} {}", pattern, button_cost);
            }
        }
        best_button_count
    }
    solve_power_requirements_rec(&machine_config.power_requirements, &patterns)
}

fn main() {
    let machine_configs: Vec<LightSchema> = read_input_day_10("input.txt");
    let mut totals: u64 = 0;
    for machine_config in machine_configs.iter() {
        let res = solve_power_requirement(machine_config);
        println!("{res}");
        totals += res;
    }
    println!("Total : {totals}");
}