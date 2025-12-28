use std::fs;

fn map_row_symbols_to_bool(row: String) -> Vec<bool> {
    row.chars().map(|x| if x == '@' {true} else {false}).collect()
}

fn visulize_matrix(m: Vec<Vec<bool>>) {
    for i in 0..(m.len()) {
        println!("{:?}", &m[i])
    }
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn count_accesible_rolls(m: &Vec<Vec<bool>>) -> Vec<(isize, isize)> {
    let mut accessible: Vec<(isize, isize)> = Vec::new();
    let mut counter: u8 = 0;
    for i in 0 .. m.len() {
        for j in 0 .. m[i as usize].len() {
            if m[i][j] {
                for (di, dj) in NEIGHBORS {
                    let ii = i as isize + di;
                    let jj = j as isize + dj;

                    if ii >= 0
                        && jj >= 0
                        && ii < m.len() as isize
                        && jj < m[ii as usize].len() as isize
                        && m[ii as usize][jj as usize]
                    {
                        counter += 1;
                    }
                }
                if counter < 4 {
                    accessible.push((i as isize,j as isize));
                }
                counter = 0;
            }
        }
    }
    accessible
}

fn count_accessible_iter(m: Vec<Vec<bool>>) -> u64 {
    let mut accessible: Vec<(isize, isize)>;
    let mut rolls: u64 = 0;
    let mut m_mut = m.clone();
    loop {
        accessible = count_accesible_rolls(&m_mut);
        if accessible.len() == 0{
            break;
        }
        rolls += accessible.len() as u64;
        for (i,j) in accessible{
            m_mut[i as usize][j as usize] = false;
        }
    }
    rolls
}

fn main() {
    let file_path = "./input.txt";
    let text_input: String = fs::read_to_string(file_path).unwrap();
    let input_matrix_text: Vec<String>  = text_input
                                        .lines()
                                        .map(String::from)
                                        .collect();
    let input_matrix: Vec<Vec<bool>> = input_matrix_text.into_iter().map(map_row_symbols_to_bool).collect();
    // visulize_matrix(input_matrix.clone());
    let accessible: Vec<(isize, isize)> = count_accesible_rolls(&input_matrix);
    println!("[Part 1] Accessible rolls: {}", accessible.len());
    let accessible_2: u64 = count_accessible_iter(input_matrix);
    println!("[Part 2] Accessible rolls: {}", accessible_2);
}
