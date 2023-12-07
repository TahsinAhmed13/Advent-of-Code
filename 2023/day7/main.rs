use std::io::{self, BufRead}; 
use std::cmp; 

fn cmp_hands1(a: &str, b: &str) -> cmp::Ordering {
    const CARDS: &str = "23456789TJQKA"; 
    let (mut ca, mut cb) = ([0; CARDS.len()], [0; CARDS.len()]); 
    for ch in a.chars() {
        let pos = CARDS.find(ch).unwrap(); 
        ca[pos] += 1; 
    }
    ca.sort_by(|a, b| b.cmp(a)); 
    for ch in b.chars() {
        let pos = CARDS.find(ch).unwrap(); 
        cb[pos] += 1; 
    } 
    cb.sort_by(|a, b| b.cmp(a)); 
    for i in 0..CARDS.len() {
        if ca[i] > cb[i] {
            return cmp::Ordering::Greater;  
        } else if ca[i] < cb[i] {
            return cmp::Ordering::Less; 
        } 
    }
    for i in 0..a.len() {
        let x = CARDS.find(a.chars().nth(i).unwrap()).unwrap(); 
        let y = CARDS.find(b.chars().nth(i).unwrap()).unwrap(); 
        if x > y {
            return cmp::Ordering::Greater; 
        } else if x < y {
            return cmp::Ordering::Less; 
        } 
    } 
    return cmp::Ordering::Equal; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut hands = Vec::<(&str, u32)>::new(); 
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        hands.push((tokens[0], tokens[1].parse::<u32>().unwrap())); 
    }
    hands.sort_by(|&(a, _), &(b, _)| cmp_hands1(a, b)); 
    let mut ans = 0; 
    for (i, (_, x)) in hands.iter().enumerate() {
        ans += (i+1) as u32 * x;  
    }
    return ans; 
}

fn cmp_hands2(a: &str, b: &str) -> cmp::Ordering {
    const CARDS: &str = "J23456789TQKA"; 
    let (mut ca, mut cb) = ([0; CARDS.len()], [0; CARDS.len()]); 
    for ch in a.chars() {
        let pos = CARDS.find(ch).unwrap(); 
        if ch != 'J' { 
            ca[pos] += 1; 
        }
    }
    ca.sort_by(|a, b| b.cmp(a)); 
    for ch in a.chars() {
        if ch == 'J' {
            ca[0] += 1; 
        }
    }
    for ch in b.chars() {
        let pos = CARDS.find(ch).unwrap(); 
        if ch != 'J' {
            cb[pos] += 1; 
        }
    } 
    cb.sort_by(|a, b| b.cmp(a)); 
    for ch in b.chars() {
        if ch == 'J' {
            cb[0] += 1; 
        }
    }
    for i in 0..CARDS.len() {
        if ca[i] > cb[i] {
            return cmp::Ordering::Greater;  
        } else if ca[i] < cb[i] {
            return cmp::Ordering::Less; 
        } 
    }
    for i in 0..a.len() {
        let x = CARDS.find(a.chars().nth(i).unwrap()).unwrap(); 
        let y = CARDS.find(b.chars().nth(i).unwrap()).unwrap(); 
        if x > y {
            return cmp::Ordering::Greater; 
        } else if x < y {
            return cmp::Ordering::Less; 
        } 
    } 
    return cmp::Ordering::Equal; 
} 

fn part2(input: &Vec<String>) -> u32 {
    let mut hands = Vec::<(&str, u32)>::new(); 
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        hands.push((tokens[0], tokens[1].parse::<u32>().unwrap())); 
    }
    hands.sort_by(|&(a, _), &(b, _)| cmp_hands2(a, b)); 
    let mut ans = 0; 
    for (i, (_, x)) in hands.iter().enumerate() {
        ans += (i+1) as u32 * x;  
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