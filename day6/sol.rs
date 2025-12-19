use std::fs;
const FILE_PATH : &str = "./input.txt";
fn main () {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    
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
        println!("total {}", total);
        grand_total += total;
    }
    return grand_total;
}
