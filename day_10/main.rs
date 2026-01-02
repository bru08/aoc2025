use std::fs;

#[derive(Debug)]
struct LightSchema {
    lights: Vec<u8>,
    buttons: Vec<Vec<usize>>,
    power_requirements: Vec<u8>
}

fn read_input_day_10(filename: &str) {
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
        let mut power_requirements: Vec<u8> = Vec::new();
        for item in light_input.iter() {
            if item.starts_with('[') {
                let content = &item[1..item.len()-1];
                light = content.chars().map(|x| if x=='.' {0_u8} else {1_u8}).collect();
            } else if item.starts_with('(') {
                let content = &item[1..item.len()-1];
                buttons.push(content.split(",").map(|x| x.parse::<usize>().unwrap()).collect());
            } else if item.starts_with('{') {
                let content = &item[1..item.len()-1];
                power_requirements = content.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
            } else {
                panic!("Unrecognised token in element {}", item)
            }
        }
        res.push(LightSchema{lights:light, buttons:buttons, power_requirements:power_requirements});
    }
    println!("{:#?}", res);
}

fn main() {
    read_input_day_10("input_test.txt");
}