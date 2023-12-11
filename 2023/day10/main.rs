use std::io::{self, BufRead}; 
use std::cmp::{max}; 
use std::collections::{VecDeque, HashSet, HashMap}; 

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1]; 
const PIPES: [[char; 3]; 4] = [
    ['F', 'L', '-'],
    ['7', 'J', '-'],
    ['F', '7', '|'],
    ['L', 'J', '|'] 
];  
const DIR: [[usize; 3]; 4] = [
    [3, 2, 0],
    [3, 2, 1],
    [1, 0, 2],
    [1, 0, 3]
];

fn get_start(input: &Vec<String>) -> Option<(usize, usize)> {
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                return Some((i, j)); 
            }
        }
    }
    return None;  
}

fn get_loop_dist(input: &Vec<String>) -> HashMap<(usize, usize), u32> {
    let (y, x) = get_start(input).unwrap(); 
    let mut dq = VecDeque::<(usize, usize, usize, u32)>::new(); 
    for i in 0..4 {
        dq.push_back((y, x, i, 0));  
    }
    let mut dist = HashMap::<(usize, usize), u32>::new(); 
    while !dq.is_empty() {
        let (y, x, dir, steps) = dq.pop_front().unwrap(); 
        if dist.contains_key(&(y, x)) && steps > 0 {
            continue; 
        }
        dist.insert((y, x), steps); 
        let (ny, nx) = (y as i32 + DY[dir], x as i32 + DX[dir]); 
        let in_bounds = 0 <= ny && ny < (input.len() as i32) && 
            0 <= nx && nx < (input[y].len() as i32);  
        let (ny, nx) = (ny as usize, nx as usize); 
        if !in_bounds {
            continue; 
        }  
        let ch = input[ny].chars().nth(nx).unwrap(); 
        for i in 0..3 {
            if ch == PIPES[dir][i] {
                dq.push_back((ny, nx, DIR[dir][i], steps+1));  
            }
        } 
    }
    return dist; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut ans = 0; 
    for (_, steps) in get_loop_dist(input) {
        ans = max(ans, steps); 
    }
    return ans; 
}

fn part2(_input: &Vec<String>) -> u32 {
    return 0; 
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