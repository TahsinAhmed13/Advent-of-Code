use std::io::{self, BufRead}; 
use std::collections::{HashMap, VecDeque}; 

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
    for (src, list) in adj {
        for dest in list {
            match rev.get_mut(dest) {
                Some(arr) => arr.push(src.to_string()),
                None => {
                    let mut arr = Vec::<String>::new();  
                    arr.push(src.to_string()); 
                    rev.insert(dest.to_string(), arr);  
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
) -> HashMap<String, (u32, u32)> {
    let rev = reverse_graph(adj);  
    let mut signals = HashMap::<String, (u32, u32)>::new(); 
    let mut dq = VecDeque::<&str>::new(); 
    for dest in adj.get("broadcaster").unwrap() {
        dq.push_back(dest); 
        if !signals.contains_key(dest) {
            signals.insert(dest.to_string(), (0, 0)); 
        }
        let signal = signals.get_mut(dest).unwrap(); 
        *signal = (signal.0+1, signal.1); 
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
                Some(val) => cnt += if !val { 1 } else { 0 }, 
            } 
        }
        let state = states.get_mut(node).unwrap(); 
        *state = get_next_state(*kind, *state, cnt); 
        for dest in adj.get(node).unwrap() {
            if !signals.contains_key(dest) {
                signals.insert(dest.to_string(), (0, 0)); 
            }
            let signal = signals.get_mut(dest).unwrap(); 
            *signal = if !*state { (signal.0+1, signal.1) } 
                else { (signal.0, signal.1+1)}; 
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
    return signals; 
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
        lows += 1; 
        for (_, (x, y)) in push_btn(&adj, &kinds, &mut states) {
            lows += x; 
            highs += y; 
        }
    } 
    return lows * highs; 
}

fn part2(input: &Vec<String>) -> u32 {
    let adj = parse_graph(input); 
    let kinds = parse_kinds(input); 
    let mut states = HashMap::<String, bool>::new(); 
    for (key, _) in &kinds {
        states.insert(key.to_string(), false); 
    }   
    let mut iter = 0;  
    loop {
        iter += 1; 
        let signals = push_btn(&adj, &kinds, &mut states);  
        match signals.get("rx") {
            None => (),
            Some(signal) => {
                if signal.0 > 0 {
                    break; 
                } 
            }
        } 
    }
    return iter; 
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