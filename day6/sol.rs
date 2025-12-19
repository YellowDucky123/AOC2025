use std::fs;
const FILE_PATH : &str = "./test.txt";
fn main () {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let lines_part2 : Vec<&str> = contents.lines().collect();

    println!("part2 : {:?}", lines_part2);
    
    let lines: Vec<Vec<&str>> = contents.lines() 
                                        .map(|line| {
                                            line.split_whitespace()             
                                                .collect()             
                                        })
                                        .filter(|v: &Vec<&str>| !v.is_empty())     
                                        .collect();                 
    println!("{:?}", lines);
    let part1_grand_total = part1(lines.clone());
    println!("part 1 : {}", part1_grand_total);
}

fn part2(lines : Vec<&str>) -> u64 {
    let mut grand_total : u64 = 0;

    let width = lines[0].len();
    let height = lines.len() - 1;

    let mut start_of_equation : bool = true;

    let mut curr_sign = "";
    let mut curr_equation_size = 1;
    let mut curr_eq_idx = 0;
    let mut curr_total : u64 = 0;
    for w in 0..width {
        if start_of_equation {
            curr_sign = lines[lines.len() - 1][w];
            
            let mut i = w + 1;
            while lines[lines.len() - 1][i] == " " {
                curr_equation_size += 1;
            } 
            curr_equation_size = i;

            start_of_equation = false;
        }
        
        if curr_eq_idx == curr_equation_size {
            curr_equation_size = 1;
            curr_eq_idx = 0;
            grand_total += curr_total;
            curr_total = 0;
            continue;
        }
        
        let mut curr_number = 0;
        for h in 0..height {
            if lines[h][w] != " " {
                curr_number *= 10;
                curr_number += match lines[h][w].parse::<u64>() {
                    Ok(number) => number,
                    Err(_e) => {
                        println!("p2 Failed to parse {}", lines[h][w]);
                        return 0;
                    }
                };
            }
        }

        curr_total += curr_number;
    }
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
