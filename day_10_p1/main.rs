use std::fs;
use std::collections::HashMap;

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

fn get_button_sequence(machine_config: &LightSchema) -> usize {
    /*Wrt v1, button press sequence does not matter. Also 2 press of same button, equals to 0 presses.
    So target solutions press each button exactly 0 or 1 times.
    */
    let mut min_result_len: u64 = u64::MAX;
    let max_combinations: i32 = 2_i32.pow(machine_config.buttons.len() as u32);
    let mut best_config: Vec<u8> = Vec::new();
    let mut solution_found: bool = false;

    for i in 0..max_combinations {
        let mut light_config_mut: Vec<u8> = vec![0_u8; machine_config.lights.len()];
        let mut buttons_config: Vec<u8> = Vec::new();
        let mut ii: i32 = i;
        let mut buttons_pressed = 0;
        for _ in 0..machine_config.buttons.len() {
            buttons_config.push((ii%2) as u8);
            ii /= 2;
        }
        //println!("{:?} {}", buttons_config, machine_config.buttons.len());
        for k in 0..buttons_config.len() {
            if buttons_config[k]==1 {
                buttons_pressed += 1;
                //println!("{k}");
                for &j in machine_config.buttons[k].iter() {
                    light_config_mut[j] = (light_config_mut[j] + 1) % 2;
                }
            }
        }
        if light_config_mut == machine_config.lights && buttons_pressed < min_result_len {
            min_result_len = buttons_pressed;
            best_config = buttons_config;
            solution_found = true;
        }

    }
    min_result_len as usize
    
}

fn part1(machine_configs: &Vec<LightSchema>) {
    let res: usize = machine_configs.iter().map(|x| {
        get_button_sequence(x)
    }).sum();
    println!("[Part 1]  Total button presses: {res}");
}

fn main() {
    let machine_configs: Vec<LightSchema> = read_input_day_10("input.txt");
    part1(&machine_configs);
}