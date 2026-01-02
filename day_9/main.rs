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

fn validate_square_position(p1_id: usize, p2_id: usize, points: &Vec<Point>) -> bool {
    let path1: Vec<usize>;
    let path2: Vec<usize>;
    if p1_id < p2_id {
        path1 = ((p1_id+1)..p2_id).collect();
        path2 = ((p2_id+1)..points.len()).chain(0..p1_id).collect();
    } else if p1_id > p2_id {
        path1 = ((p2_id+1)..p1_id).collect();
        path2 = ((p1_id+1)..points.len()).chain(0..p2_id).collect();
    } else {
        return true
    }
    let mut path1_upper: Vec<u8> = Vec::new();
    let mut path2_upper: Vec<u8> = Vec::new();
    let p1:&Point = &points[p1_id];
    let p2:&Point = &points[p2_id];
    let max_x: i64 = p1.x.max(p2.x);
    let min_x: i64 = p1.x.min(p2.x);
    let max_y: i64 = p1.y.max(p2.y);
    let min_y: i64 = p1.y.min(p2.y);


    for &p_id in path1.iter() {
        let point: &Point = &points[p_id];
        if point.x >= min_x && point.x <= max_x {
            if point.y >= max_y {
                path1_upper.push(1);
            } else if point.y <= min_y {
                path1_upper.push(2);
            } else {
                path1_upper.push(0);
            }
        }
    }

    for &p_id in path2.iter() {
        let point: &Point = &points[p_id];
        if point.x >= min_x && point.x <= max_x {
            if point.y >= max_y {
                path2_upper.push(1);
            } else if point.y <= min_y {
                path2_upper.push(2);
            }  else {
                path2_upper.push(0);
            }
        }
    }

    //println!("paths {} {}", path1.len(), path1_upper.len());
    if path1_upper.len() == 0 || path2_upper.len() == 0 {return false};
    let path_1_result: Option<u8> = path1_upper.iter().all(|&x| x == path1_upper[0]).then(|| path1_upper[0]);
    let path_2_result: Option<u8> = path2_upper.iter().all(|&x| x == path2_upper[0]).then(|| path2_upper[0]);
    // println!("{:?}\n{:?}", path1_upper, path2_upper);
    // println!("\n");

    match (path_1_result, path_2_result) {
        (Some(a), Some(b)) => {
            if a != b && a != 0 && b!= 0 {true} else {false}
        },
        _ => false
    }
}

fn part2(points: &Vec<Point>) -> i64 {
    // this works for test input but not real input of day 9
    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            if validate_square_position(i, j, &points) {
                let area = Point::area(&points[i], &points[j]);
                max_area = max_area.max(area);
            }
        }
    }
    max_area
}

fn coordinate_compression(points: &Vec<Point>) -> Vec<Point> {
    let mut x_vec: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut y_vec: Vec<i64> = points.iter().map(|p| p.y).collect();
    x_vec.sort();
    y_vec.sort();
    let mut cc_points: Vec<Point> = Vec::new();
    for p in points {
        cc_points.push(Point{
            x: x_vec.iter().position(|&elem| elem == p.x).unwrap() as i64,
            y: y_vec.iter().position(|&elem| elem == p.y).unwrap() as i64
        })
    }
    cc_points
}

fn points_map(points: &Vec<Point>) -> Vec<Vec<bool>> {
    let x_max = points.iter().map(|p| p.x).max().unwrap() as usize;
    let y_max = points.iter().map(|p| p.y).max().unwrap() as usize;
    let mut map_vec = vec![vec![false;x_max+1]; y_max+1];
    for p in points.iter(){ 
        map_vec[p.y as usize][p.x as usize] = true 
    }
    map_vec
}

fn points_connect(point_map: &mut Vec<Vec<bool>>, points: &Vec<Point>) {
    for ii in 0..=points.len() {
        let i1: usize = ii%points.len();
        let i2: usize = (ii+1)%points.len();
        if points[i1].x == points[i2].x {
            for j in points[i1].y.min(points[i2].y)..=points[i1].y.max(points[i2].y) {
                point_map[j as usize][points[i1].x as usize] = true;
            }
        } else if points[i1].y == points[i2].y {
            for j in points[i1].x.min(points[i2].x)..=points[i1].x.max(points[i2].x) {
                point_map[points[i1].y as usize][j as usize] = true;
            }
        } else {
            panic!("Unkwonst status, there should be at least one aligne point.")
        }
    }
}

fn validate_rect_position_cc(points_map: &Vec<Vec<bool>>, p1: &Point, p2: &Point) -> bool {
    let x_min = p1.x.min(p2.x) as usize;
    let x_max = p1.x.max(p2.x) as usize;
    let y_min = p1.y.min(p2.y) as usize;
    let y_max = p1.y.max(p2.y) as usize;
    
    points_map[y_min..=y_max]
        .iter()
        .flat_map(|row| &row[x_min..=x_max])
        .all(|&x| x)
}

fn inner_point_polygon(point_map:&Vec<Vec<bool>>) -> Option<Point> {
    // Check intersection of rays with supposedly closed polygon, return first available inner point
    let mut am_in: bool = false;
    let mut last_intersection: usize = 0;
    for i in 0..point_map.len(){
        am_in = false;
        for j in 0..point_map[i].len() {
            if point_map[i][j] {
                if !am_in {
                    am_in = true;
                    last_intersection=j;
                } else if am_in && j > last_intersection + 1 {
                    return Some(Point {x:(last_intersection+1) as i64, y:i as i64})
                } else if am_in && j <= last_intersection + 1 {
                    am_in = false;
                } else {
                    panic!("Unrecognised ray case");
                }
            };
        }
    }
    None
}

fn flood_fill(points_map: &mut Vec<Vec<bool>>, starting_point: Point) {
    let mut stack = Vec::new();
    stack.push(starting_point);
    
    while let Some(point) = stack.pop() {
        let x = point.x;
        let y = point.y;
        
        // Bounds check
        if x < 0 || y < 0 
            || y as usize >= points_map.len() 
            || x as usize >= points_map[0].len() 
            || points_map[y as usize][x as usize] {
            continue;
        }
        
        // Mark as filled
        points_map[y as usize][x as usize] = true;
        
        // Add neighbors to stack
        stack.push(Point { x: x + 1, y });
        stack.push(Point { x: x - 1, y });
        stack.push(Point { x, y: y + 1 });
        stack.push(Point { x, y: y - 1 });
    }
}


fn part2_cc(points: &Vec<Point>) -> i64 {
    // This solution take inspiration from several solutions hints about using coordinate compression
    let mut max_area: i64 = 0;
    let points_cc: Vec<Point> = coordinate_compression(points);
    let mut map_cc: Vec<Vec<bool>> = points_map(&points_cc);
    // for row in map_cc.iter() { println!("{:?}", row.iter().map(|&x| x as u8).collect::<Vec<u8>>())}

    points_connect(&mut map_cc, &points_cc);

    // println!(""); for row in map_cc.iter() { println!("{:?}", row.iter().map(|&x| x as u8).collect::<Vec<u8>>())}

    let starting_point: Point = inner_point_polygon(&map_cc).unwrap();
    
    flood_fill(&mut map_cc, starting_point);

    // println!(""); for row in map_cc.iter() { println!("{:?}", row.iter().map(|&x| x as u8).collect::<Vec<u8>>())}
    
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            if validate_rect_position_cc(&map_cc, &points_cc[i], &points_cc[j]) {
                let area = Point::area(&points[i], &points[j]);
                max_area = max_area.max(area);
            }
        }
    }
    max_area
}


fn main() {
    let filename = "./input.txt";
    let points: Vec<Point> = fs::read_to_string(filename)
                                .unwrap()
                                .lines()
                                .map(String::from)
                                .map(|x| {
                                    let (x1, x2) = x.split_once(',').unwrap();
                                    Point{x: x1.parse::<i64>().unwrap(), y: x2.parse::<i64>().unwrap()}
                                })
                                .collect();
    println!("[Part 1] Max area      {}", part1(&points));
    println!("[Part 2] Max area      {}", part2(&points));
    println!("[Part 2] Max area (cC) {}", part2_cc(&points));
}