use std::fs;
const FILE_PATH : &str = "./input.txt";
fn main () {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let lines_part2 : Vec<&str> = contents.lines().collect();

    let lines: Vec<Vec<&str>> = contents.lines() 
                                        .map(|line| {
                                            line.split_whitespace()             
                                                .collect()             
                                        })
                                        .filter(|v: &Vec<&str>| !v.is_empty())     
                                        .collect();                 
    let part1_grand_total = part1(lines.clone());
    println!("part 1 : {}", part1_grand_total);

    let part2_grand_total = part2(lines_part2.clone());
    println!("part 2 : {}", part2_grand_total);
}

fn part2(lines : Vec<&str>) -> u64 {
    let mut grand_total : u64 = 0;

    let width = lines[0].len();
    let height = lines.len() - 1;

    let mut start_of_equation : bool = true;

    let mut curr_sign : u8 = b' ';
    let mut curr_equation_size = 0;
    let mut curr_eq_idx = 0;
    let mut curr_total : u64 = 0;
    for w in 0..width {
        if start_of_equation {
            curr_sign = lines[lines.len() - 1].as_bytes()[w];

            if curr_sign == b'*' {
                curr_total = 1;
            }
            
            let mut i = w + 1;
            while i < width && lines[lines.len() - 1].as_bytes()[i] == b' ' {
                curr_equation_size += 1;
                i += 1;
            } 

            if i == width {
                curr_equation_size += 1;
            }

            start_of_equation = false;
        }

        if curr_eq_idx == curr_equation_size {
            curr_equation_size = 0;
            curr_eq_idx = 0;
            grand_total += curr_total;
            curr_total = 0;
            start_of_equation = true;
            continue;
        }
         
        let mut curr_number : u64 = 0;
        for h in 0..height {
            if lines[h].as_bytes()[w] != b' ' {
                curr_number *= 10_u64;
                curr_number += (lines[h].as_bytes()[w] - b'0') as u64;
            }
        }

        if curr_sign == b'+' {
            curr_total += curr_number;
        } else if curr_sign == b'*' {
            curr_total *= curr_number;
        }
        curr_eq_idx += 1;   
    }

    grand_total += curr_total;

    return grand_total;
}

fn part1(lines : Vec<Vec<&str>>) -> u64 {
    let mut grand_total : u64 = 0;

    let num_equation = lines[0].len();

    
    for eq_idx in 0..num_equation {
        let sign = lines[lines.len() - 1][eq_idx];

        let mut total : u64 = 0;
        if sign == "*" {
            total = 1;
        }

        for eq_element in 0..(lines.len() - 1) {
            let curr_num_str = lines[eq_element][eq_idx];
            let curr_num = match curr_num_str.parse::<u64>() {
                Ok(number) => {
                    number
                },
                Err(_e) => {
                    println!("parse fail {}", curr_num_str);
                    return 0;
                }
            };
            
            if sign == "+" {
                total += curr_num;
            } else if sign == "*" {
                total *= curr_num;
            }
        }
        //println!("total {}", total);
        grand_total += total;
    }
    return grand_total;
}
