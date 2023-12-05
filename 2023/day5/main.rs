use std::io::{self, BufRead}; 
use std::cmp::{min, max}; 

fn get_seeds(input: &str) -> Vec<u64> {
    let tokens: Vec<&str> = input.split_whitespace().collect(); 
    let mut seeds = Vec::<u64>::new(); 
    for i in 1..tokens.len() {
        seeds.push(tokens[i].parse::<u64>().unwrap()); 
    }
    return seeds; 
}

fn get_map(input: &[String]) -> Vec<(u64, u64, u64)> {
    let mut map = Vec::<(u64, u64, u64)>::new();  
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let (x, y, z) = (
            tokens[0].parse::<u64>().unwrap(), 
            tokens[1].parse::<u64>().unwrap(), 
            tokens[2].parse::<u64>().unwrap()
        ); 
        map.push((y, y+z-1, x));  
    }
    map.sort(); 
    return map; 
}

fn get_maps(input: &[String]) -> Vec<Vec<(u64, u64, u64)>> {
    let mut maps = Vec::<Vec<(u64, u64, u64)>>::new();  
    let mut last = 0; 
    for (i, line) in input.iter().enumerate() {
        if line == "" && i != last {
            maps.push(get_map(&input[last+2..i]));  
            last = i; 
        }
    } 
    maps.push(get_map(&input[last+2..]));  
    return maps; 
}

fn part1(alamac: &Vec<String>) -> u64 {
    let maps = get_maps(&alamac[1..]); 
    let mut loc = u64::MAX; 
    for seed in get_seeds(&alamac[0]) {
        let mut val = seed; 
        for map in &maps {
            for range in map {
                if range.0 <= val && val <= range.1 {
                    val = range.2 + (val - range.0);  
                    break; 
                } 
            }
        }
        loc = min(loc, val); 
    }
    return loc; 
}

fn merge_ranges(from: &Vec<(u64, u64)>, to: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut ans = Vec::<(u64, u64)>::new(); 
    let mut j = 0;  
    for i in 0..from.len() {
        let mut last = from[i].0; 
        while j < to.len() && to[j].1 < from[i].0 {
            j += 1; 
        }
        while j < to.len() && to[j].0 <= from[i].1 {
            if last < to[j].0 {
                ans.push((last, to[j].0 - 1)); 
            }
            let left = max(last, to[j].0); 
            let right = min(from[i].1, to[j].1); 
            ans.push((to[j].2 + (left - to[j].0), to[j].2 + (right - to[j].0))); 
            last = to[j].1 + 1; 
            j += 1; 
        }
        if last <= from[i].1 {
            ans.push((last, from[i].1)); 
        }
        if j > 0 {
            j -= 1; 
        }
    } 
    ans.sort();  
    return ans; 
}

fn part2(alamac: &Vec<String>) -> u64 {
    let seeds = get_seeds(&alamac[0]); 
    let mut ranges = Vec::<(u64, u64)>::new(); 
    for i in (0..seeds.len()).step_by(2) {
        ranges.push((seeds[i], seeds[i] + seeds[i+1] - 1)); 
    }
    ranges.sort(); 
    for map in &get_maps(&alamac[1..]) {
        ranges = merge_ranges(&ranges, &map); 
    }
    return ranges[0].0;  
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