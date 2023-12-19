use std::io::{self, BufRead}; 
use std::cmp::{min}; 

const DIRECTIONS: &str = "URDL"; 
const DY: [i64; 4] = [1, 0, -1, 0]; 
const DX: [i64; 4] = [0, 1, 0, -1]; 

fn get_volume(ys: &Vec<i64>, xs: &Vec<i64>) -> i64 {
    let n = min(ys.len(), xs.len()); 
    let (mut area, mut boundary) = (0, 0); 
    for i in 0..n {
        area += ys[i] * xs[(i+1)%xs.len()]; 
        area -= xs[i] * ys[(i+1)%ys.len()]; 
        boundary += (ys[(i+1)%ys.len()] - ys[i]).abs(); 
        boundary += (xs[(i+1)%xs.len()] - xs[i]).abs(); 
    } 
    let interior = (area.abs() - boundary + 2) / 2; 
    return boundary + interior; 
}

fn part1(input: &Vec<String>) -> u64 {
    let (mut ys, mut xs) = (Vec::<i64>::new(), Vec::<i64>::new()); 
    let (mut y, mut x) = (0, 0); 
    for line in input {
        ys.push(y); 
        xs.push(x); 
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let dir = DIRECTIONS.find(tokens[0]).unwrap();  
        let amt = tokens[1].parse::<i64>().unwrap(); 
        y += amt * DY[dir]; 
        x += amt * DX[dir]; 
    }
    return get_volume(&ys, &xs) as u64;  
}

fn parse_color(hex: &str) -> (usize, i64) {
    let val = i64::from_str_radix(&hex[1..hex.len()-1], 16).unwrap(); 
    let ch = hex.chars().last().unwrap(); 
    let dir = ch.to_digit(10).unwrap() as usize; 
    return (dir, val); 
}

fn part2(input: &Vec<String>) -> u64 {
    let (mut ys, mut xs) = (Vec::<i64>::new(), Vec::<i64>::new()); 
    let (mut y, mut x) = (0, 0); 
    for line in input {
        ys.push(y); 
        xs.push(x); 
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let (dir, amt) = parse_color(&tokens[2][1..tokens[2].len()-1]);  
        y += amt * DY[dir]; 
        x += amt * DX[dir]; 
    }
    return get_volume(&ys, &xs) as u64; 
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