use std::io::{self, BufRead}; 

fn get_wins(time: u64, dist: u64) -> u64 {
    let mut cnt = 0; 
    for j in 0..time+1 {
        if j*(time-j) > dist {
            cnt += 1; 
        }
    }
    return cnt; 
}

fn part1(input: &Vec<String>) -> u64 {
    let time_tokens: Vec<&str> = input[0].split_whitespace().collect(); 
    let dist_tokens: Vec<&str> = input[1].split_whitespace().collect(); 
    let mut ans = 1; 
    for i in 1..time_tokens.len() {
        let time = time_tokens[i].parse::<u64>().unwrap(); 
        let dist = dist_tokens[i].parse::<u64>().unwrap();     
        ans *= get_wins(time, dist);  
    } 
    return ans; 
}

fn part2(input: &Vec<String>) -> u64 {
    let (mut time, mut dist): (u64, u64) = (0, 0); 
    for ch in input[0].chars() {
        if ch.is_digit(10) {
            let x = ch.to_digit(10).unwrap() as u64; 
            time = 10*time + x; 
        }
    } 
    for ch in input[1].chars() {
        if ch.is_digit(10) {
            let x = ch.to_digit(10).unwrap() as u64; 
            dist = 10*dist + x; 
        }
    }
    return get_wins(time, dist); 
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