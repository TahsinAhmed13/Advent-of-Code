use std::io::{self, BufRead}; 
use std::collections::HashSet; 
use std::cmp::min; 

fn get_matches(card: &str) -> u32 {
    let tokens: Vec<&str> = card.split_whitespace().collect(); 
    let mut cur = 2;
    let mut win = HashSet::<u32>::new(); 
    while cur < tokens.len() && tokens[cur] != "|" {
        win.insert(tokens[cur].parse::<u32>().unwrap()); 
        cur += 1; 
    }
    cur += 1; 
    let mut cnt = 0;  
    while cur < tokens.len() {
        let x = tokens[cur].parse::<u32>().unwrap(); 
        if win.contains(&x) {
            cnt += 1; 
        } 
        cur += 1; 
    }
    return cnt; 
}

fn get_value(matches: u32) -> u32 {
    if matches == 0 {
        return 0; 
    }
    return 1<<(matches-1); 
}

fn part1(cards: &Vec<String>) -> u32 {
    let mut total = 0; 
    for card in cards {
        total += get_value(get_matches(&card)); 
    }
    return total; 
}

fn part2(cards: &Vec<String>) -> u32 {
    let mut cnt = vec![1; cards.len()]; 
    let mut total = 0; 
    for i in 0..cards.len() {
        let matches = get_matches(&cards[i]); 
        let to = min(i + (matches as usize) + 1, cards.len()); 
        total += cnt[i]; 
        for j in i+1..to {
            cnt[j] += cnt[i];  
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