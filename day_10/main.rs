use std::fs;

#[derive(Debug)]
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

// fn button_action(light_config: &Vec<u8>, button: &Vec<usize>) -> Vec<u8> {
//     // modify light config using button spec
//     let mut light_config_mut: Vec<u8> = vec![0_u8; machine_config.lights.len()];
//     for &i in button.iter() {
//         light_config[i] = (light_config[i] + 1) % 2;
//     }
// }

fn get_button_sequence(machine_config: &LightSchema) -> usize {
    let mut stack: Vec<Vec<usize>> = Vec::new();
    let mut results: Vec<Vec<usize>> = Vec::new();
    let mut min_result_len: u64 = u64::MAX;
    for next_button_id in 0..machine_config.buttons.len() {
            stack.push(vec![next_button_id]);
    }

    while let Some(button_seq) = stack.pop() {
        let mut light_config_mut: Vec<u8> = vec![0_u8; machine_config.lights.len()];
        for &i in button_seq.iter() {
            for &j in machine_config.buttons[i].iter() {
                
                light_config_mut[j] = (light_config_mut[j] + 1) % 2;
                //println!("inupdate: {:?} {:?} {} {}",button_seq,light_config_mut, light_config_mut[j], j );
            }
        }
        // println!("{:?} || {:?}",light_config_mut, machine_config.lights);
        if light_config_mut == machine_config.lights && (button_seq.len() as u64) < min_result_len {
            results.push(button_seq.clone());
            min_result_len = button_seq.len() as u64;
        } else  if button_seq.len() < 7{
            for next_button_id in 0..machine_config.buttons.len() {
                let mut new_seq = button_seq.clone();
                new_seq.push(next_button_id);
                stack.push(new_seq)
            }
        }
    }
    results.iter().map(|x| x.len()).min().unwrap()
}

fn part1(machine_configs: &Vec<LightSchema>) {
    let res: usize = machine_configs.iter().enumerate().map(|(i,x)| {
        println!("Proc row {i}");
        get_button_sequence(x)
    }).sum();
    println!("[Part 1]  Total button presses: {res}");
}


fn get_button_sequence_joltage(machine_config: &LightSchema) -> usize {
    let mut stack: Vec<Vec<usize>> = Vec::new();
    let mut results: Vec<Vec<usize>> = Vec::new();
    let mut min_result_len: u64 = u64::MAX;
    for next_button_id in 0..machine_config.buttons.len() {
            stack.push(vec![next_button_id]);
    }

    while let Some(button_seq) = stack.pop() {
        let mut joltage_counters_mut: Vec<u64> = vec![0_u64; machine_config.power_requirements.len()];
        let mut its_over: bool = false;
        'buttons: for &i in button_seq.iter() {
            //println!("{:?}", joltage_counters_mut);
            'button_spec: for &j in machine_config.buttons[i].iter() {
                joltage_counters_mut[j]+=1;
                if  joltage_counters_mut[j] > machine_config.power_requirements[j] {
                    its_over=true;
                    break 'buttons;
                }
        }
        if joltage_counters_mut == machine_config.power_requirements && (button_seq.len() as u64) < min_result_len {
            results.push(button_seq.clone());
            min_result_len = button_seq.len() as u64;
        } else  if button_seq.len() < 15 && !its_over {
            for next_button_id in 0..machine_config.buttons.len() {
                let mut new_seq = button_seq.clone();
                new_seq.push(next_button_id);
                stack.push(new_seq);
            }
        }
    }
    
    }
    results.iter().map(|x| x.len()).min().unwrap()
}

fn part2(machine_configs: &Vec<LightSchema>) {
    let res: usize = machine_configs.iter().enumerate().map(|(i,x)| {
        println!("Proc row {i}");
        get_button_sequence_joltage(x)
    }).sum();
    println!("[Part 2]  Total button presses: {res}");
}



fn main() {
    let machine_configs: Vec<LightSchema> = read_input_day_10("input_test.txt");
    // part1(&machine_configs);
    part2(&machine_configs);
}