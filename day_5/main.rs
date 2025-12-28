use std::fs;
use std::collections::HashSet;

fn count_fresh_ids(mut product_ranges: Vec<(u64, u64)>) -> u64 {
    /*Count total elements from possibily overlapping batch of u64 ranges.
    First sort them by start of the range, Then iteratively merge or add to the collection if non overlapping.
    Then count element for each range and sum.
    */
    product_ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(u64, u64)> = vec![product_ranges[0]];
    for (lower, upper) in product_ranges {
        let last_idx = merged.len() -1;
        let (last_lower, last_upper) = merged[last_idx];
        if lower <= last_upper {
            merged[last_idx] = (last_lower, last_upper.max(upper));
        } else {
            merged.push((lower, upper));
        }
    }
    merged.iter().map(|(lower, upper)| upper-lower+1).sum()
}

fn main() {
    let file_path = "./input.txt";
    let text_input: String = fs::read_to_string(file_path).unwrap();
    let (ranges_input, pids_input) = text_input.split_once("\n\n").unwrap();
    let product_ids: Vec<u64> = pids_input.lines().map(|x| x.parse::<u64>().unwrap()).collect(); 
    let product_ranges: Vec<(u64, u64)> = ranges_input.lines().map(|x| {
        let (a,b) = x.split_once('-').unwrap();
        (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
    }).collect();
    let mut valid_pid_counter: u64 = 0;
    'pid_loop : for &pid in product_ids.iter() {
        'pid_range_loop : for &(low_bound, upper_bound) in product_ranges.iter() {
            if pid >= low_bound && pid <= upper_bound {
                valid_pid_counter += 1;
                break 'pid_range_loop;
            }
        }
    }
    let fresh_ids: u64 = count_fresh_ids(product_ranges);
    println!("[Part 1] Valid Product ids: {valid_pid_counter}");
    println!("[Part 2] Total fresh ids: {fresh_ids}");
}