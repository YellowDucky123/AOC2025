use std::fs;

const FILE_PATH : &str = "./input.txt";

fn main() {
    println!("start AOC!!\n");

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read file");

    let instructions : Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();

    let mut password : i32 = 0;
    let mut dial : i32 = 50;
    for instruction in &instructions {
        password += dial_rotation(&mut dial, instruction);
    }

    println!("Password is {password}");
}

fn dial_rotation(dial : &mut i32, instruction : &str) -> i32 {
    let mut letters = instruction.chars();
    
    let direction = letters.next().unwrap();
    let number_chars : String = letters.collect(); 
    let mut number = number_chars.parse::<i32>().unwrap();
    let mut zeroes = number / 100;
        
    number %= 100; 
    
    if direction == 'L' { 
        if number >= *dial {
            zeroes += 1;
        }
        *dial -= number;
    } else if direction == 'R' {
        if number >= (100 - *dial) {
            zeroes += 1;
        }
        *dial += number;    
    }
    *dial = (*dial % 100 + 100) % 100;

    return zeroes;
}
