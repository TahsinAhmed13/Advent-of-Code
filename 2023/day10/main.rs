use std::io::{self, BufRead}; 
use std::cmp::{min}; 

#[derive(Copy, Clone, Debug)]
enum DIRECTION { UP, RIGHT, DOWN, LEFT } 
impl DIRECTION {
    const VALUES: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]; 
}

const DY: [isize; 4] = [-1, 0, 1, 0]; 
const DX: [isize; 4] = [0, 1, 0, -1]; 
const PIPES: [[char; 3]; 4] = [
    ['F', '7', '|'],
    ['7', 'J', '-'],
    ['L', 'J', '|'], 
    ['F', 'L', '-'],
];  
const DMAP: [[DIRECTION; 3]; 4] = [
    [DIRECTION::RIGHT, DIRECTION::LEFT, DIRECTION::UP],
    [DIRECTION::DOWN, DIRECTION::UP, DIRECTION::RIGHT], 
    [DIRECTION::RIGHT, DIRECTION::LEFT, DIRECTION::DOWN],
    [DIRECTION::DOWN, DIRECTION::UP, DIRECTION::LEFT]
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

fn get_next(input: &Vec<String>, (y, x, dir): (usize, usize, DIRECTION)) -> Option<(usize, usize, DIRECTION)> {
    let (r, c) = (input.len(), input[0].len()); 
    let (ny, nx) = (y as isize + DY[dir as usize], x as isize + DX[dir as usize]); 
    let in_bounds = 0 <= ny && ny < (r as isize) && 0 <= nx && nx < (c as isize);  
    if !in_bounds {
        return None; 
    }
    let (ny, nx) = (ny as usize, nx as usize); 
    let ch = input[ny].chars().nth(nx).unwrap(); 
    for i in 0..PIPES[dir as usize].len() {
        if ch == PIPES[dir as usize][i] {
            return Some((ny, nx, DMAP[dir as usize][i])); 
        }
    }
    return None; 
}

fn get_path(input: &Vec<String>) -> Vec<(usize, usize)> {
    let (sy, sx) = get_start(input).unwrap(); 
    let mut path = Vec::<(usize, usize)>::new(); 
    let mut cur = None; 
    for dir in DIRECTION::VALUES {
        let next = get_next(input, (sy, sx, dir)); 
        if next.is_some() {
            cur = next; 
            break; 
        }
    }
    while cur.is_some() {
        let (y, x, dir) = cur.unwrap(); 
        path.push((y, x)); 
        cur = get_next(input, (y, x, dir)); 
    }
    path.push((sy, sx)); 
    return path; 
}

fn part1(input: &Vec<String>) -> u32 {
    let path_len = get_path(input).len() as u32; 
    return path_len / 2; 
}

fn get_interior(ys: &Vec<i32>, xs: &Vec<i32>) -> u32 {
    let n = min(ys.len(), xs.len()); 
    let (mut area, mut boundary) = (0, 0); 
    for i in 0..n {
        area += ys[i] * xs[(i+1)%xs.len()]; 
        area -= xs[i] * ys[(i+1)%ys.len()]; 
        boundary += (ys[(i+1)%ys.len()] - ys[i]).abs(); 
        boundary += (xs[(i+1)%xs.len()] - xs[i]).abs(); 
    } 
    return ((area.abs() - boundary + 2) / 2) as u32; 
}

fn part2(input: &Vec<String>) -> u32 {
    let (mut ys, mut xs) = (Vec::<i32>::new(), Vec::<i32>::new()); 
    for (y, x) in get_path(input) {
        let ch = input[y].chars().nth(x).unwrap();  
        if ch != '|' && ch != '-' {
            ys.push(y as i32); 
            xs.push(x as i32); 
        }
    }
    return get_interior(&ys, &xs);  
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