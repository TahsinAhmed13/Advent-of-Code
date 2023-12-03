use std::io::{self, BufRead}; 

fn part1(engine: &Vec<String>) -> u32 {
    const DX: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1]; 
    const DY: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1]; 
    let mut total = 0; 
    for i in 0..engine.len() {
        let (mut val, mut pw) = (0, 1); 
        let mut ok = false;  
        for (j, ch) in engine[i].chars().rev().enumerate() {
            if ch.is_numeric() {
                val += ch.to_digit(10).unwrap() * pw; 
                pw *= 10; 
                for k in 0..8 {
                    let (ni, nj) = (i as i32 + DX[k], (engine[i].len()-j-1)  as i32 + DY[k]); 
                    let in_bounds = 0 <= ni && ni < engine.len() as i32 &&
                        0 <= nj && nj < engine[i].len() as i32; 
                    if !in_bounds { 
                        continue; 
                    }
                    let sym = engine[ni as usize].chars().nth(nj as usize).unwrap(); 
                    if sym != '.' && !sym.is_numeric() {
                        ok = true; 
                    } 
                }
            } else {
                total += if ok { val } else { 0 }; 
                val = 0; 
                pw = 1; 
                ok = false; 
            }
        }
        total += if ok { val } else { 0 }; 
    }
    return total; 
}

fn get_number(line: &str, index: usize) -> u32 {
    if index >= line.len() || !line.chars().nth(index).unwrap().is_numeric() {
        return 0; 
    }
    let (mut left, mut right) = (index, index+1); 
    while left > 0 && line.chars().nth(left-1).unwrap().is_numeric() {
        left -= 1; 
    } 
    while right < line.len() && line.chars().nth(right).unwrap().is_numeric() {
        right += 1; 
    } 
    return line[left..right].parse::<u32>().unwrap();  
}

fn add_numbers(nums: &mut Vec<u32>, line: &str, index: usize) {
    let (left, mid, right) = (
        get_number(line, index-1),
        get_number(line, index),
        get_number(line, index+1)    
    );  
    if mid > 0 {
        nums.push(mid); 
    } else {
        if left > 0 {
            nums.push(left); 
        }
        if right > 0 {
            nums.push(right); 
        }
    }
}

fn part2(engine: &Vec<String>) -> u32 {
    let mut total = 0; 
    for (i, line) in engine.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '*' {
                let mut nums = Vec::<u32>::new(); 
                if i > 0 {
                    add_numbers(&mut nums, &engine[i-1], j); 
                }  
                add_numbers(&mut nums, line, j); 
                if i < engine.len()-1 {
                    add_numbers(&mut nums, &engine[i+1], j); 
                }
                if nums.len() == 2 {
                    total += nums[0] * nums[1]; 
                }
            }
        }
    }
    return total; 
}

fn main() {
    let mut input = Vec::new(); 
    let reader = io::stdin().lock(); 
    for line in reader.lines() {
        input.push(line.unwrap()); 
    }    
    println!("{}", part1(&input)); 
    println!("{}", part2(&input)); 
}
