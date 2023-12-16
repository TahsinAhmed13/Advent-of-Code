use std::io::{self, BufRead}; 
use std::collections::HashMap; 

fn hash(s: &str) -> u32 {
    let mut res = 0; 
    for ch in s.chars() {
        res += ch as u32; 
        res *= 17; 
        res %= 256; 
    }
    return res; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut res = 0; 
    for token in input[0].split(',') {
        res += hash(token);  
    }
    return res; 
}

fn part2(input: &Vec<String>) -> u32 {
    let mut cnt = [0; 256]; 
    let mut pos = Vec::<HashMap<String, usize>>::new();  
    let mut vals = Vec::<HashMap<String, u32>>::new();  
    for _ in 0..256 {
        pos.push(HashMap::<String, usize>::new()); 
        vals.push(HashMap::<String, u32>::new()); 
    }
    for instr in input[0].split(',') {
        let tokens: Vec<&str> = instr.split('=').collect(); 
        if tokens.len() > 1 {
            let idx = hash(tokens[0]) as usize; 
            let lens = tokens[1].parse::<u32>().unwrap(); 
            if !pos[idx].contains_key(tokens[0]) {
                pos[idx].insert(tokens[0].to_string(), cnt[idx]); 
                cnt[idx] += 1;  
            }
            vals[idx].insert(tokens[0].to_string(), lens); 
        } else {
            let token = &instr[..instr.len()-1]; 
            let idx = hash(token) as usize; 
            pos[idx].remove(token);
            vals[idx].remove(token);  
        } 
    } 
    let mut res = 0; 
    for i in 0..256 {
        let mut all = Vec::<(usize, u32)>::new(); 
        for (s, p) in &pos[i] {
            match vals[i].get(s) {
                None => continue,
                Some(lens) => all.push((*p, *lens)), 
            }
        }
        all.sort(); 
        for j in 0..all.len() {
            res += (i as u32 + 1) * (j as u32 + 1) * all[j].1; 
        }
    } 
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