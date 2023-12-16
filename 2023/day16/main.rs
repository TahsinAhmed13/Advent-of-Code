use std::io::{self, BufRead}; 
use std::cmp::{max}; 
use std::collections::{VecDeque, HashSet}; 

const UP: usize = 0; 
const RIGHT: usize = 1; 
const DOWN: usize = 2; 
const LEFT: usize = 3; 
const DY: [isize; 4] = [-1, 0, 1, 0]; 
const DX: [isize; 4] = [0, 1, 0, -1]; 
const MIRRORS: &str = ".-|/\\"; 

fn get_next((r, c): (usize, usize), (y, x, dir): (usize, usize, usize)) -> Option<(usize, usize)> {
    let (ny, nx) = (y as isize + DY[dir], x as isize + DX[dir]); 
    let in_bounds = 0 <= ny && ny < r as isize && 0 <= nx && nx < c as isize; 
    if in_bounds {
        return Some(((ny as usize), (nx as usize))); 
    }
    return None; 
}

fn get_energized(input: &Vec<String>, (sy, sx, sdir): (usize, usize, usize)) -> HashSet<(usize, usize)> {
    let splits: [[Vec<usize>; 4]; 5] = [
        [vec![UP], vec![RIGHT], vec![DOWN], vec![LEFT]],
        [vec![RIGHT,LEFT], vec![RIGHT], vec![RIGHT,LEFT], vec![LEFT]],
        [vec![UP], vec![UP,DOWN], vec![DOWN], vec![UP,DOWN]],
        [vec![RIGHT], vec![UP], vec![LEFT], vec![DOWN]],
        [vec![LEFT], vec![DOWN], vec![RIGHT], vec![UP]] 
    ];

    let (r, c) = (input.len(), input[0].len()); 
    let mut energized = HashSet::<(usize, usize)>::new(); 
    let mut seen = HashSet::<(usize, usize, usize)>::new(); 
    let mut dq = VecDeque::<(usize, usize, usize)>::new(); 
    dq.push_back((sy, sx, sdir)); 
    while !dq.is_empty() {
        let (y, x, dir) = dq.pop_front().unwrap(); 
        if seen.contains(&(y, x, dir)) {
            continue; 
        }
        seen.insert((y, x, dir)); 
        energized.insert((y, x)); 
        let ch = input[y].chars().nth(x).unwrap(); 
        let pos = MIRRORS.find(ch).unwrap();  
        for ndir in &splits[pos][dir] {
            match get_next((r, c), (y, x, *ndir)) {
                None => continue,
                Some((ny, nx)) => dq.push_back((ny, nx, *ndir)),
            }
        }
    } 
    return energized; 
}

fn part1(input: &Vec<String>) -> u32 {
    let energized = get_energized(input, (0, 0, RIGHT)); 
    return energized.len() as u32; 
}

fn part2(input: &Vec<String>) -> u32 {
    let mut res = 0; 
    for i in 0..input.len() {
        res = max(res, get_energized(input, (i, 0, RIGHT)).len() as u32); 
        res = max(res, get_energized(input, (i, input[i].len()-1, LEFT)).len() as u32); 
    }
    for j in 0..input[0].len() {
        res = max(res, get_energized(input, (0, j, DOWN)).len() as u32); 
        res = max(res, get_energized(input, (input[0].len()-1, j, UP)).len() as u32); 
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