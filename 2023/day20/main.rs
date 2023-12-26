use std::io::{self, BufRead}; 
use std::collections::{HashMap, HashSet, VecDeque}; 

fn parse_graph(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut adj = HashMap::<String, Vec<String>>::new(); 
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect();  
        let kind = tokens[0].chars().nth(0).unwrap();  
        let src = if kind == '%' || kind == '&' 
            { tokens[0][1..].to_string() } else { tokens[0].to_string() }; 
        let mut edges = Vec::<String>::new(); 
        for i in 2..tokens.len() {
            let dest = tokens[i].split(',').nth(0).unwrap().to_string();  
            edges.push(dest); 
        }
        adj.insert(src, edges);  
    }    
    return adj; 
}

fn parse_kinds(input: &Vec<String>) -> HashMap<String, bool> {
    let mut states = HashMap::<String, bool>::new();  
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let kind = tokens[0].chars().nth(0).unwrap();
        if kind == '%' {
            states.insert(tokens[0][1..].to_string(), false); 
        } else if kind == '&' {
            states.insert(tokens[0][1..].to_string(), true); 
        }
    }
    return states; 
}

fn reverse_graph(adj: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut rev = HashMap::<String, Vec<String>>::new(); 
    for (src, neighbors) in adj {
        for dest in neighbors {
            match rev.get_mut(dest) {
                Some(list) => list.push(src.to_string()),
                None => {
                    let list = vec![src.to_string()]; 
                    rev.insert(dest.to_string(), list);  
                }
            }
        }
    }
    return rev; 
}

fn get_next_state(kind: bool, old_state: bool, cnt: u32) -> bool {
    if !kind {
        return !old_state; 
    }    
    return cnt != 0; 
}

fn push_btn(
    adj: &HashMap<String, Vec<String>>, 
    kinds: &HashMap<String, bool>, 
    states: &mut HashMap<String, bool>
) -> (u32, u32) {
    let rev = reverse_graph(adj);  
    let (mut lows, mut highs) = (0, 0); 
    let mut dq = VecDeque::<&str>::new(); 
    lows += 1; 
    for dest in adj.get("broadcaster").unwrap() {
        lows += 1; 
        dq.push_back(dest); 
    } 
    while !dq.is_empty() {
        let node = dq.pop_front().unwrap(); 
        if !kinds.contains_key(node) {
            continue; 
        }  
        let kind = kinds.get(node).unwrap(); 
        let mut cnt = 0;
        for src in rev.get(node).unwrap() {
            match states.get(src) {
                None => continue,
                Some(val) => cnt += if !*val { 1 } else { 0 }, 
            } 
        }
        let state = states.get_mut(node).unwrap(); 
        *state = get_next_state(*kind, *state, cnt); 
        for dest in adj.get(node).unwrap() {
            lows += if !*state { 1 } else { 0 }; 
            highs += if *state { 1 } else { 0 }; 
            match kinds.get(dest) {
                None => continue,
                Some(kind) => {
                    if *kind || !*state {
                        dq.push_back(&dest);  
                    }       
                }
            }
        }
    }
    return (lows, highs); 
}

fn part1(input: &Vec<String>) -> u32 {
    const ITERATIONS: u32 = 1000; 
    let adj = parse_graph(input); 
    let kinds = parse_kinds(input); 
    let mut states = HashMap::<String, bool>::new(); 
    for (key, _) in &kinds {
        states.insert(key.to_string(), false); 
    }
    let (mut lows, mut highs) = (0, 0); 
    for _ in 0..ITERATIONS {
        let (x, y) = push_btn(&adj, &kinds, &mut states); 
        lows += x; 
        highs += y; 
    } 
    return lows * highs; 
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
    let adj = parse_graph(input); 
    let kinds = parse_kinds(input); 
    let mut dp = HashMap::<String, u64>::new(); 
    let mut deg = HashMap::<String, usize>::new(); 
    for (node, neighbors) in reverse_graph(&adj) {
        match kinds.get(&node) {
            None => continue,
            Some(kind) => {
                if *kind {
                    deg.insert(node, neighbors.len()); 
                }
            }
        }
    }
    let mut vis = HashSet::<String>::new(); 
    let mut dq = VecDeque::<&str>::new(); 
    for dest in adj.get("broadcaster").unwrap() {
        dp.insert(dest.to_string(), 1);  
        dq.push_back(dest); 
    }
    while !dq.is_empty() {
        let node = dq.pop_front().unwrap();  
        vis.insert(node.to_string()); 
        if !kinds.contains_key(node) {
            continue; 
        }
        let from = kinds.get(node).unwrap(); 
        let cur = dp.get(node).unwrap().clone(); 
        for dest in adj.get(node).unwrap() {
            if vis.contains(dest) {
                continue; 
            }
            let to = kinds.get(dest).unwrap_or(&false); 
            if !*from {
                if !*to {
                    dp.insert(dest.to_string(), cur<<1);
                } else if !dp.contains_key(dest) {
                    dp.insert(dest.to_string(), cur); 
                } else {
                    let val = dp.get_mut(dest).unwrap(); 
                    *val += cur; 
                }
            } else {
                if !dp.contains_key(dest) {
                    dp.insert(dest.to_string(), cur); 
                } else {
                    let val = dp.get_mut(dest).unwrap(); 
                    *val = lcm(*val, cur);  
                }
            } 
            if deg.contains_key(dest) {
                let cnt = deg.get_mut(dest).unwrap(); 
                *cnt -= 1;
            }
            if *deg.get(dest).unwrap_or(&0) == 0 {
                dq.push_back(dest); 
            }
        }
    }
    return *dp.get("rx").unwrap(); 
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