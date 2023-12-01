use std::io::{self, BufRead}; 

fn part1(calibration: &Vec<String>) -> u32 {
    let mut ans = 0; 
    for s in calibration {
        let (mut first, mut last) = (0, 0); 
        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap(); 
            if ch.is_numeric() {
                first = ch.to_digit(10).unwrap();   
                break; 
            } 
        }
        for i in (0..s.len()).rev() {
            let ch = s.chars().nth(i).unwrap(); 
            if ch.is_numeric() {
                last = ch.to_digit(10).unwrap(); 
                break; 
            }
        }
        ans += 10 * first + last; 
    }
    return ans; 
}

fn get_numstr(s: &str, i: usize) -> u32 {
    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 
    for j in 0..NUMBERS.len() {
        if i+NUMBERS[j].len() > s.len() {
            continue; 
        }
        let ss = &s[i..i+NUMBERS[j].len()]; 
        if ss == NUMBERS[j] {
            return (j+1) as u32; 
        }  
    }
    return 0; 
}

fn part2(calibration: &Vec<String>) -> u32 {
    let mut ans = 0; 
    for s in calibration {
        let (mut first, mut last) = (0, 0); 
        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap(); 
            let num = get_numstr(s, i); 
            if ch.is_numeric() {
                first = ch.to_digit(10).unwrap();   
                break; 
            } else if num > 0 {
                first = num; 
                break; 
            }  
        }
        for i in (0..s.len()).rev() {
            let ch = s.chars().nth(i).unwrap(); 
            let num = get_numstr(s, i); 
            if ch.is_numeric() {
                last = ch.to_digit(10).unwrap(); 
                break; 
            } else if num > 0 {
                last = num; 
                break; 
            }
        }
        ans += 10 * first + last; 
    }
    return ans; 
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