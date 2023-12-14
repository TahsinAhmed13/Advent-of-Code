use std::io::{self, BufRead}; 

fn is_vmatch(input: &Vec<Vec<char>>, pos: usize) -> bool {
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

fn is_hmatch(input: &Vec<Vec<char>>, pos: usize) -> bool {
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
        if is_vmatch(input, i) {
            res += i as u32 + 1; 
        }
    }
    for i in 0..input.len()-1 {
        if is_hmatch(input, i) {
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

fn summarize2(input: &Vec<Vec<char>>) -> u32 {
    let mut clone = Vec::<Vec<char>>::new(); 
    for line in input{
        clone.push(line.clone()); 
    }      
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            clone[i][j] = if input[i][j] == '#' { '.' } else { '#' }; 
            for k in 0..input[0].len()-1 {
                if is_vmatch(&clone, k) && !is_vmatch(input, k) {
                    return k as u32 + 1; 
                }
            }
            for k in 0..input.len()-1 {
                if is_hmatch(&clone, k) && !is_hmatch(input, k) {
                    return 100 * (k as u32 + 1); 
                }
            }
            clone[i][j] = input[i][j]; 
        }
    }
    return 0; 
}

fn part2(input: &Vec<String>) -> u32 {
    let mut res = 0; 
    let mut pattern = Vec::<Vec<char>>::new(); 
    for line in input {
        if line.is_empty() {
            res += summarize2(&pattern); 
            pattern = Vec::<Vec<char>>::new(); 
        } else {
            pattern.push(line.chars().collect::<Vec<char>>()); 
        }
    }
    res += summarize2(&pattern); 
    return res; 
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