use std::io::{self, BufRead}; 

fn is_vertical_match(input: &Vec<Vec<char>>, pos: usize) -> bool {
    let mut i = 0; 
    while pos >= i && pos+i+1 < input[0].len() {
        for j in 0..input.len() {
            if input[j][pos-i] != input[j][pos+i+1] {
                return false; 
            }
        }
        i += 1; 
    } 
    return true; 
}

fn is_horizontal_match(input: &Vec<Vec<char>>, pos: usize) -> bool {
    let mut i = 0; 
    while pos >= i && pos+i+1 < input.len() {
        for j in 0..input[i].len() {
            if input[pos-i][j] != input[pos+i+1][j] {
                return false; 
            }
        }
        i += 1; 
    } 
    return true; 
}

fn summarize(input: &Vec<Vec<char>>) -> u32 {
    let mut res = 0; 
    for i in 0..input[0].len()-1 {
        if is_vertical_match(input, i) {
            res += i as u32 + 1; 
        }
    }
    for i in 0..input.len()-1 {
        if is_horizontal_match(input, i) {
            res += 100 * (i as u32 + 1); 
        }
    } 
    return res; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut res = 0; 
    let mut pattern = Vec::<Vec<char>>::new(); 
    for line in input {
        if line.is_empty() {
            res += summarize(&pattern); 
            pattern = Vec::<Vec<char>>::new(); 
        } else {
            pattern.push(line.chars().collect::<Vec<char>>()); 
        }
    }
    res += summarize(&pattern); 
    return res; 
}

fn main() {
    let mut input = Vec::new(); 
    let reader = io::stdin().lock(); 
    for line in reader.lines() {
        input.push(line.unwrap()); 
    }    
    println!("{}", part1(&input)); 
    // println!("{}", part2(&input)); 
}