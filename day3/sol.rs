use std::fs;
const FILE_PATH : &str = "./input.txt";
fn main () {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let lines : Vec<&str> = contents.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    let mut banks : Vec<Vec<i8>> = Vec::new();

    for line in &lines {
        let mut bank : Vec<i8> = Vec::new();
        for battery in line.chars() {
            bank.push((battery as i8) - ('0' as i8));
        }
        banks.push(bank);
    }
    
    let total_jolt = part1(banks.clone());
    println!("total part 1 : {}", total_jolt);

    let total_jolt_pt2 = part2(banks.clone());
    println!("total part 2 : {}", total_jolt_pt2);
}

fn biggest (arr : Vec<i8>, number_order : i8) -> (i8, usize){ 
    let mut biggest : i8 = 0;
    let mut biggest_id : usize = 0;

    let mut max_len = arr.len() - 1;
    if number_order == 2 {
        max_len = arr.len();
    }

    for bat_id in 0..max_len {
        if arr[bat_id] > biggest {
            biggest = arr[bat_id];
            biggest_id = bat_id;
        }
    }
    return (biggest, biggest_id);
}  

fn biggest_pt2 (arr : Vec<i8>, number_order : usize) -> (u64, usize) {
    let mut biggest : i8 = 0;
    let mut biggest_id : usize = 0;
   

    let max_len = arr.len() - number_order;
    for bat_id in 0..max_len {
        if arr[bat_id] > biggest {
            biggest = arr[bat_id];
            biggest_id = bat_id;
        }
    }

    return (biggest as u64, biggest_id);
}

fn part2 (banks : Vec<Vec<i8>>) -> u64 {
    let mut total : u64 = 0;
    
    for bank in &banks {
        let mut jolt : u64 = 0;
        let mut current_id : usize = 0;
        
        for i in (0..12).rev() {
            let (num, num_id) = biggest_pt2(bank[current_id..].to_vec(), i);
        
            jolt += num * (10_u64.pow(i as u32));
         
            current_id += num_id + 1;
        }
        total += jolt;
    }

    return total;
}

fn part1 (banks : Vec<Vec<i8>>) -> i32 {
    let mut total : i32 = 0;
    for bank in &banks {
        let (first, first_id) = biggest(bank.clone(), 1);
        let (second, _second_id) = biggest(bank[(first_id + 1)..].to_vec(), 2);
        
        let jolt : i32 = ((first as i32) * 10) + (second as i32);
        total += jolt;
    }

    return total;
}
