use std::fs;

#[derive(Debug)]
struct JBox {
    x: i64,
    y: i64,
    z: i64,
    connected: bool
}
impl JBox {
    fn new(x:i64, y:i64, z:i64) -> Self {
        JBox {
            x,
            y,
            z,
            connected: false
        }
    }

    fn dist(a: &JBox, b:&JBox) -> f64 {
        ((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)).isqrt() as f64
    }

}

fn main() {
    let file_path = "./input_test.txt";
    let text_input: Vec<Vec<i64>> = fs::read_to_string(file_path)
                                        .unwrap()
                                        .lines()
                                        .map(String::from)
                                        .map(|x| x.split(',').map(|y| y.parse::<i64>().unwrap()).collect())
                                        .collect();
    let mut boxes: Vec<JBox> = text_input.into_iter().map(|x| JBox::new(x[0], x[1], x[2])).collect();
    let mut distances: Vec<Vec<f64>> = Vec::new();
    for i in 0..boxes.len(){
        distances.push(Vec::new());
        for j in (i+1)..boxes.len(){
            distances[i].push(JBox::dist(&boxes[i], &boxes[j]));
        }
    }
    println!("{:?}", boxes);
    println!("{:?}", distances);
}