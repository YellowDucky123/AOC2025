use std::fs;
const FILE_PATH : &str = "./input.txt";
fn main () {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let ranges : Vec<&str> = contents.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    let mut total : i64 = 0;
    let mut total_2 : i64 = 0;

    for range in &ranges {
        let mut bounds : Vec<i64> = Vec::new();

        let str_bounds : Vec<&str> = range.split('-').collect();

        for bound in str_bounds {
            let num = bound.parse::<i64>().unwrap();
            bounds.push(num);
        }
        total += part1(bounds.clone());
        total_2 += part2(bounds.clone());
    }

    println!("total is {}", total);
    println!("total 2 is {}", total_2);
}

fn string_check(s: String) -> bool {
    for length in 1..=(s.len() / 2) {
        if s.len() % length != 0 {
            continue;
        }

        let mut str_match = true;
        let substr = &s[0..length];
        //println!("string being checked: {}, with substr {}", s, substr);
        let mut i = length;
        
        while i < s.len() { 
            //println!("string {}, substr {}", &s[i..(i + length)], substr);
            if &s[i..(i + length)] != substr {
                str_match = false;
                break;
            }
                
            i += length;
        }
        if str_match {
            return true;
        }
    }
    return false;
}

fn part2 (bounds : Vec<i64>) -> i64 {
    let mut total : i64 = 0;

    println!("{} - {} : ", bounds[0], bounds[1]);
    for i in bounds[0]..=bounds[1] {
        let string_ver = i.to_string();

        if string_check(string_ver) {
            println!("{}", i);
            total += i;
        }
    }

    return total;
}

fn part1 (bounds : Vec<i64>) -> i64 {
    let mut total : i64 = 0;
    for i in bounds[0]..=bounds[1] {
        let string_ver = i.to_string();
        
        if string_ver.len() % 2 != 0 {
            continue;
        }
        
        let first_half = &string_ver[0..(string_ver.len() / 2)];
        let second_half = &string_ver[(string_ver.len() / 2)..string_ver.len()];

        if first_half == second_half {
            total += i;
        }
    }

    return total;
}
