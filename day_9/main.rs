use std::fs;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}
impl Point {

    fn area(p1: &Point, p2: &Point) -> i64{
        ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1)
    }
}

fn part1(points: &Vec<Point>) -> i64 {
    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let area = Point::area(&points[i], &points[j]);
            //println!("{:?} - {:?} = {}", points[i], points[j], area);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn validate_square_position(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    /*
    let mut path1_contains: bool = false;
    let mut path2_contains: bool = false;
    if p1_id <= p2_id {
        let path1: Vec<usize> = (p1_id+1..p2).collect();
        let path2: Vec<usize> = (p2..points.len()).collect().extend((0..p1).collect());
    } else {
        let path1: Vec<usize> = p2_id+1..p1;
        let path2: Vec<usize> = (p1..points.len()).collect().extend((0..p2).collect());
    }
    */
    for point in points.iter(){
        if (point.x < p1.x.max(p2.x) && point.x > p1.x.min(p2.x)) && (point.y < p1.y.max(p2.y) && point.y > p1.y.min(p2.y)) {
        }
    }
    true
}

fn part2(points: &Vec<Point>) -> i64 {
    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            if validate_square_position(&points[i], &points[j], &points) {
                let area = Point::area(&points[i], &points[j]);
                max_area = max_area.max(area);
                println!("Valid square of area {} between points {:?} {:?}", area, &points[i], &points[j]);
            }
        }
    }
    max_area
}


fn main() {
    let filename = "./input_test.txt";
    let points: Vec<Point> = fs::read_to_string(filename)
                                .unwrap()
                                .lines()
                                .map(String::from)
                                .map(|x| {
                                    let (x1, x2) = x.split_once(',').unwrap();
                                    Point{x: x1.parse::<i64>().unwrap(), y: x2.parse::<i64>().unwrap()}
                                })
                                .collect();
    println!("[Part 1] Max area {}", part1(&points));
    println!("[Part 2] Max area {}", part2(&points));
}