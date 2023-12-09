use std::io::{self, BufRead}; 

fn parse_histories(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut histories = Vec::<Vec<i64>>::new(); 
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let mut history = Vec::<i64>::new(); 
        for num in tokens {
            history.push(num.parse::<i64>().unwrap()); 
        } 
        histories.push(history.clone()); 
    }
    return histories; 
}

fn extrapolate_right(history: &Vec<i64>) -> i64 {
    if history.len() == 0 {
        return 0; 
    }
    let mut diff = Vec::<i64>::new(); 
    let mut done = true; 
    for i in 0..history.len()-1 {
        done = done && history[i] == 0; 
        diff.push(history[i+1]-history[i]); 
    }
    let last = history.last().copied().unwrap();  
    if done && last == 0 {
        return 0; 
    }
    return last + extrapolate_right(&diff); 
}

fn part1(input: &Vec<String>) -> i64 {
    let mut total = 0; 
    for history in parse_histories(input) {
        total += extrapolate_right(&history); 
    } 
    return total; 
}

fn extrapolate_left(history: &Vec<i64>) -> i64 {
    if history.len() == 0 {
        return 0; 
    }
    let mut diff = Vec::<i64>::new(); 
    let mut done = true; 
    for i in 1..history.len() {
        done = done && history[i] == 0; 
        diff.push(history[i]-history[i-1]); 
    }
    let first = history.first().copied().unwrap();  
    if done && first == 0 {
        return 0; 
    }
    return first - extrapolate_left(&diff); 
}

fn part2(input: &Vec<String>) -> i64 {
    let mut total = 0; 
    for history in parse_histories(input) {
        total += extrapolate_left(&history); 
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