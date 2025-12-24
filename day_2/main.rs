use std::fs;

/*
fn range_str_to_int(range_str: String) -> Vec<u32> {
    // convert a string like "12-15" to [12, 15]
    // It is general purprose, and can create arbitrary length vectors from string separated by dash sign
    let range_limits: Vec<u32> = range_str.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
    range_limits
}
*/

fn verify_product_id(product_id: String) -> bool {
    // Verify a product id and output if is valid (true) or invalid (false)
    // Check if product id is made of a substring repeated twice
    if product_id.len() % 2 == 0{
        if product_id[0 .. product_id.len() / 2] == product_id[product_id.len() / 2 .. product_id.len()]{
            return false
        }
    }
    true
}

fn verify_product_id_v2(product_id: String) -> bool {
    // Verify a product id and output if is valid (true) or invalid (false)
    // Check if product id is made of a substring repeated twice
    for window_length in 1..(1 + product_id.len() / 2) {
        if product_id.len() % window_length == 0 {
            let mut equal_count = 1;
            for j in 1..(product_id.len()/window_length){
                if product_id[0..window_length] == product_id[(j)*window_length..(j+1)*window_length]{
                    equal_count += 1;
                }
            }
            if equal_count == (product_id.len()/window_length){
                // if condition of equality is true for some window length then product_id invalid, exit function.
                // println!("Here2 {product_id} not valid");
                return false
            }
        }
    }
    true
}

fn check_product_id_range(proudct_id_range: String, verify_function: fn(String)->bool) -> Vec<String> {
    /*
    Interpret a string based range representation "<start>-<end>" and iterate generating productids inbetween.
    Verfy them and return the invalid ones.

    PS:
    Another way of converting: ( start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap() )
    */
    let (start, end) = proudct_id_range.split_once('-').unwrap();
    let start: u64 = start.parse().unwrap();
    let end: u64 = end.parse().unwrap();
    let mut invalid_ids: Vec<String> = vec![];
    for product_id_num in start..(end + 1) {
        if !verify_function(product_id_num.to_string()){
            invalid_ids.push(product_id_num.to_string());
        }
    }
    invalid_ids
}


fn main() {
    let file_path = "./input.txt";
    let ranges: Vec<String> = fs::read_to_string(file_path)
                                    .unwrap()
                                    .split(',')
                                    .map(String::from)
                                    .collect();

    let invalid_ids: Vec<String> = ranges.clone().into_iter().flat_map(|x| check_product_id_range(x, verify_product_id)).collect();
    let sum_invalid_ids: u64 = invalid_ids.clone().into_iter().map(|x| x.parse::<u64>().unwrap()).sum();
    let invalid_ids_part2: Vec<String> = ranges.clone().into_iter().flat_map(|x| check_product_id_range(x, verify_product_id_v2)).collect();
    let sum_invalid_ids_part2: u64 = invalid_ids_part2.clone().into_iter().map(|x| x.parse::<u64>().unwrap()).sum();
    println!("[Part 1] Sample of invalid ids {:?}", &invalid_ids[0..3]);
    println!("[Part 1] Sum of all invalid ids is {sum_invalid_ids}");
    println!("[Part 2] Sample of invalid ids {:?}", &invalid_ids_part2[0..3]);
    println!("[Part 2] Sum of all invalid ids is {sum_invalid_ids_part2}");
}