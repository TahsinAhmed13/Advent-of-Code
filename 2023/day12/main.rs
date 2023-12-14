use std::io::{self, BufRead}; 

fn parse_line(line: &str) -> (Vec<char>, Vec<u32>) {
    let tokens: Vec<&str> = line.split_whitespace().collect(); 
    let spring: Vec<char> = tokens[0].chars().collect();  
    let mut damages = Vec::<u32>::new(); 
    for token in tokens[1].split(',') {
        damages.push(token.parse::<u32>().unwrap());  
    } 
    return (spring, damages); 
}

fn get_arrangments(spring: &Vec<char>, damages: &Vec<u32>) -> u64 {
    let mut dp = vec![vec![0; damages.len()+1]; spring.len()+1]; 
    dp[0][0] = 1; 
    for i in 0..spring.len() {
        if spring[i] == '#' {
            break; 
        }
        dp[i+1][0] = 1; 
    }
    for i in 0..spring.len() {
        for j in 0..damages.len() {
            if spring[i] != '#' {
                dp[i+1][j+1] += dp[i][j+1]; 
            }
            if spring[i] != '.' {
                let mut k: usize = 0;
                while k < damages[j] as usize && k <= i {
                    if spring[i-k] == '.' {
                        break; 
                    }
                    k += 1; 
                }
                if k < damages[j] as usize { 
                    continue; 
                }
                if k <= i && spring[i-k] == '#' {
                    continue; 
                }
                if k <= i && spring[i-k] == '?' {
                    k += 1; 
                }
                dp[i+1][j+1] += dp[i+1-k][j]; 
            }
        }
    }
    return dp[spring.len()][damages.len()]; 
}

fn part1(input: &Vec<String>) -> u64 {
    let mut res = 0; 
    for line in input {
        let (spring, damages) = parse_line(&line); 
        res += get_arrangments(&spring, &damages); 
    }    
    return res; 
}

fn part2(input: &Vec<String>) -> u64 {
    let mut res = 0; 
    for line in input {
        let (spring, damages) = parse_line(&line); 
        let (mut spring2, mut damages2) = (spring.clone(), damages.clone()); 
        for _ in 0..4 {
            spring2.push('?'); 
            for ch in &spring {
                spring2.push(*ch); 
            } 
            for x in &damages {
                damages2.push(*x);  
            }
        } 
        res += get_arrangments(&spring2, &damages2); 
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