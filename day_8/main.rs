use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct JBox {
    x: i64,
    y: i64,
    z: i64
}
impl JBox {
    fn new(x:i64, y:i64, z:i64) -> Self {
        JBox {
            x,
            y,
            z
        }
    }

    fn dist(a: &JBox, b:&JBox) -> f64 {
        ((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)).isqrt() as f64
    }

}


fn count_size(components: &Vec<usize>, top_k: usize) -> usize {
    let mut component_sizes: HashMap<usize, usize> = HashMap::new();
    for &comp_id in components.iter() {
        *component_sizes.entry(comp_id).or_insert(0) += 1;
    }
    let mut sizes: Vec<usize> = component_sizes.values().cloned().collect::<Vec<usize>>();
    sizes.sort();
    sizes.reverse();
    let part1_result_circui_sizes: usize = sizes[..top_k].iter().product::<usize>();
    part1_result_circui_sizes
}


fn main() {
    let file_path = "./input.txt";
    let text_input: Vec<Vec<i64>> = fs::read_to_string(file_path)
                                        .unwrap()
                                        .lines()
                                        .map(String::from)
                                        .map(|x| x.split(',').map(|y| y.parse::<i64>().unwrap()).collect())
                                        .collect();
    let boxes: Vec<JBox> = text_input.into_iter().enumerate().map(|(i, x)| JBox::new(x[0], x[1], x[2])).collect();
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..boxes.len(){
        for j in (i+1)..boxes.len(){
            edges.push((
                i, 
                j, 
                JBox::dist(&boxes[i], &boxes[j])
            ));
        }
    }
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut components: Vec<usize> = (0..boxes.len()).collect();
    let mut connection_count: u64 = 0;
    let mut last_boxes: (&JBox, &JBox) = (&boxes[0],&boxes[0]);
    for edge in edges.iter() {
        if components[edge.0] != components[edge.1] {
            let value_to_update: usize = components[edge.0];
            for i in 0..components.len(){
                if components[i] == value_to_update {components[i] =  components[edge.1]}
            }
            last_boxes = (&boxes[edge.0], &boxes[edge.1]);
        }
        connection_count += 1;
        if connection_count == 10-1 { 
            println!("[Part1] Result is {}", count_size(&components, 3))
         }
    }

    println!("Last boxes {:#?}", last_boxes);
    println!("Part2 : {}", last_boxes.0.x * last_boxes.1.x);
}