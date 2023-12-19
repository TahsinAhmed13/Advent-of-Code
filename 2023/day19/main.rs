use std::io::{self, BufRead}; 
use std::collections::{HashMap}; 

fn parse_workflows(input: &[String]) -> HashMap<String, String> {
    let mut workflows = HashMap::<String, String>::new(); 
    for line in input {
        let start = line.find('{').unwrap(); 
        let name = line[..start].to_string(); 
        let rules = line[start+1..line.len()-1].to_string(); 
        workflows.insert(name, rules); 
    }
    return workflows; 
}

fn parse_systems(input: &[String]) -> Vec<[u32; 4]> {
    let mut systems = Vec::<[u32; 4]>::new(); 
    for line in input {
        let mut parts = [0; 4]; 
        for (i, token) in line[1..line.len()-1].split(',').enumerate() {
            parts[i] = token[2..].parse::<u32>().unwrap();  
        }
        systems.push(parts); 
    } 
    return systems; 
}
 
fn parse_rule(rule: &str) -> (usize, char, u32) {
    const COMPONENTS: &str = "xmas"; 
    let ch = rule.chars().nth(0).unwrap(); 
    let pos = COMPONENTS.find(ch).unwrap(); 
    let cmp = rule.chars().nth(1).unwrap(); 
    let val = rule[2..].parse::<u32>().unwrap();  
    return (pos, cmp, val); 
}

fn eval_rule(cmp: char, a: u32, b: u32) -> bool {
    return (cmp == '<' && a < b) || (cmp == '>' && a > b);  
}

fn execute_workflow(wf: &str, parts: &[u32; 4]) -> Option<String> {
    for rule in wf.split(',') {
        let tokens: Vec<&str> = rule.split(':').collect(); 
        if tokens.len() == 1 {
            return Some(tokens[0].to_string());  
        }  
        let (pos, cmp, val) = parse_rule(tokens[0]);          
        if eval_rule(cmp, parts[pos], val) {
            return Some(tokens[1].to_string());  
        }
    }
    return None; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut curs = 0;  
    while curs < input.len() && !input[curs].is_empty() {
        curs += 1; 
    }
    let workflows = parse_workflows(&input[..curs]); 
    let mut rating = 0; 
    for parts in parse_systems(&input[curs+1..]) {
        let mut state = String::from("in"); 
        while state != "A" && state != "R" {
            let wf = workflows.get(&state).unwrap(); 
            state = execute_workflow(wf, &parts).unwrap();     
        }
        if state == "A" {
            for x in parts {
                rating += x; 
            }
        }
    }
    return rating; 
}

fn part2(input: &Vec<String>) -> u64 {
    const LOW: u32 = 1; 
    const HIGH: u32 = 4000; 
    let mut curs = 0;  
    while curs < input.len() && !input[curs].is_empty() {
        curs += 1; 
    }
    let workflows = parse_workflows(&input[..curs]); 
    let mut count: u64 = 0; 
    let mut st = Vec::<([(u32, u32); 4], &str)>::new(); 
    st.push(([(LOW, HIGH); 4], "in")); 
    while !st.is_empty() {
        let (mut ranges, state) = st.pop().unwrap(); 
        if state == "A" {
            let mut amt: u64 = 1; 
            for (lo, hi) in ranges {
                amt *= (hi-lo+1) as u64; 
            }
            count += amt; 
        }
        if state == "A" || state == "R" {
            continue; 
        }
        let wf = workflows.get(state).unwrap(); 
        for rule in wf.split(',') {
            let tokens: Vec<&str> = rule.split(':').collect(); 
            if tokens.len() == 1 {
                st.push((ranges, tokens[0]));  
                break;  
            }
            let (pos, cmp, val) = parse_rule(tokens[0]); 
            let mut cpy = ranges.clone();  
            if cmp == '<' {
                cpy[pos].1 = val-1; 
                ranges[pos].0 = val; 
            } else {
                cpy[pos].0 = val+1; 
                ranges[pos].1 = val; 
            }
            st.push((cpy, tokens[1])); 
        }
    }
    return count; 
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