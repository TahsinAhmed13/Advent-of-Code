use std::io::{self, BufRead}; 
use std::collections::HashMap; 

fn parse_map(input: &[String]) -> HashMap<String, (String, String)> {
    let mut mp = HashMap::<String, (String, String)>::new();  
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect();  
        let src = tokens[0].to_string(); 
        let left = tokens[2][1..tokens[2].len()-1].to_string(); 
        let right = tokens[3][..tokens[3].len()-1].to_string(); 
        mp.insert(src, (left, right));  
    }
    return mp; 
}

fn traverse_map(instr: &str, mp: &HashMap<String, (String, String)>, start: &str) -> u64 {
    let (mut steps, mut pos) = (0, 0); 
    let mut cur = start; 
    while cur.as_bytes()[2] != b'Z' {
        if instr.as_bytes()[pos] == b'L' {
            cur = &mp[cur].0;     
        } else {
            cur = &mp[cur].1;
        }
        pos = (pos + 1) % instr.len(); 
        steps += 1; 
    }
    return steps; 
}

fn part1(input: &Vec<String>) -> u64 {
    return traverse_map(&input[0], &parse_map(&input[2..]), "AAA");  
}

fn gcd(x: u64, y: u64) -> u64 {
    if x > y {
        return gcd(y, x); 
    } else if x == 0 {
        return y; 
    }    
    return gcd(x, y%x); 
}

fn lcm(x: u64, y: u64) -> u64 {
    return (x*y) / gcd(x, y); 
}

fn part2(input: &Vec<String>) -> u64 {
    let mp = parse_map(&input[2..]); 
    let mut ans = 0; 
    for (key, _) in &mp {
        if key.as_bytes()[2] == b'A' {
            let steps = traverse_map(&input[0], &mp, key); 
            if ans > 0 {
                ans = lcm(ans, steps); 
            } else {
                ans = steps; 
            } 
        }
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