use std::fs;

fn print_schema(schema: &Vec<Vec<char>>) {
    // Utility function to print the device schema
    for i in 0..schema.len(){
        println!("{}", schema[i][..].iter().collect::<String>());
    }
}

struct BTreeNode {
    col_id: usize,
    left: Option<Box<BTreeNode>>,
    right: Option<Box<BTreeNode>>,
}

impl BTreeNode {
    fn new(col_id: usize) -> Self {
        BTreeNode {
            col_id,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, col_id: usize) {
        self.left = Some(Box::new(BTreeNode::new(col_id)));
    }
    
    fn add_right(&mut self, col_id: usize) {
        self.right = Some(Box::new(BTreeNode::new(col_id)));
    }

    fn add_split(&mut self) {
        self.left = Some(Box::new(BTreeNode::new(self.col_id-1)));
        self.right = Some(Box::new(BTreeNode::new(self.col_id+1)));
    }

    fn get_leaf_nodes(&mut self) -> Vec<&mut BTreeNode> {
        let mut leaves = Vec::new();
        self.collect_leaves(&mut leaves);
        leaves
    }

    fn collect_leaves<'a>(&'a mut self, leaves: &mut Vec<&'a mut BTreeNode>) {
        // If this is a leaf node, add it
        if self.left.is_none() && self.right.is_none() {
            leaves.push(self);
            return;
        }
        
        // Otherwise, recursively collect from children
        if let Some(left) = &mut self.left {
            left.collect_leaves(leaves);
        }
        if let Some(right) = &mut self.right {
            right.collect_leaves(leaves);
        }
    }
}

fn part1(text_input_ref: &Vec<Vec<char>>)-> u64 {
    let mut split_count: u64 = 0;
    let mut text_input = text_input_ref.clone();
    for row_id in 0..text_input.len()-1 {
        /*
        println!("Iteration {row_id}");
        print_schema(&text_input);
        println!("\n----------------------\n");
        */
        for col_id in 0..text_input[row_id].len() {
            if text_input[row_id][col_id] == '|' || text_input[row_id][col_id] == 'S' {
                if text_input[row_id+1][col_id] == '.' {
                    text_input[row_id+1][col_id] = '|'
                } else if text_input[row_id+1][col_id] == '^' {
                    split_count += 1;
                    if text_input[row_id+1][col_id-1] == '.' {text_input[row_id+1][col_id-1] = '|'}
                    if text_input[row_id+1][col_id+1] == '.' {text_input[row_id+1][col_id+1] = '|'}
                }
            }
        }
    }
    split_count
}

fn part2_btree(text_input: &Vec<Vec<char>>) -> u64 {
    let mut root_id: Option<usize> = None;
    for col_id in 0..text_input[0].len() {
       if text_input[0][col_id] == 'S' {
                root_id = Some(col_id);
                break;
        } 
    }
    let root_id = root_id.expect("Error: 'S' Beam root not found in input first line.");

    let mut tree_root: BTreeNode = BTreeNode::new(root_id);
    for row_id in 1..text_input.len() {
        println!("Processing input row {row_id}");
        for leaf in tree_root.get_leaf_nodes() {
                if text_input[row_id][leaf.col_id] == '^' { leaf.add_split() };
        }
    }
    tree_root.get_leaf_nodes().len() as u64
}

fn part2(text_input: &Vec<Vec<char>>) -> u64 {
    // Simplified part 2 version, just hover a count vector to keep track of how many differnt paths are nwo intersecating a spacific cell of the input schema.
    // finally sum total counts.
    let text_input_width: usize = text_input[0].len();
    let mut paths_bank: Vec<u64> = vec![0; text_input_width];
    for row_id in 0..text_input.len() {
        for col_id in 0..text_input_width{
            if text_input[row_id][col_id] == 'S' {
                paths_bank[col_id] = 1
            } else if text_input[row_id][col_id] == '^' {
                paths_bank[col_id - 1] += paths_bank[col_id];
                paths_bank[col_id + 1] += paths_bank[col_id];
                paths_bank[col_id] = 0;
            }
        }
    }
    paths_bank.iter().sum::<u64>()
}

fn main() {
    let file_path = "./input.txt";
    let text_input: Vec<Vec<char>> = fs::read_to_string(file_path).unwrap().lines().map(String::from).map(|x| x.chars().collect()).collect();
    let result_part1: u64 = part1(&text_input);
    // let result_part2: u64 = part2_btree(&text_input); // too slow, inefficient.
    let result_part2: u64 = part2(&text_input);
    println!("[Part 1] Total beam split: {result_part1}");
    println!("[Part 2] Total beam paths: {result_part2}");
}