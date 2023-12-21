use std::io::{self, BufRead}; 
use std::collections::{VecDeque}; 

#[derive(Copy, Clone, Debug)]
enum DIRECTION { UP, RIGHT, DOWN, LEFT } 
impl DIRECTION {
    const VALUES: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]; 
}
const DY: [isize; 4] = [-1, 0, 1, 0]; 
const DX: [isize; 4] = [0, 1, 0, -1]; 

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

fn get_next((r, c): (usize, usize), (y, x): (usize, usize), dir: DIRECTION) -> Option<(usize, usize)> {
    let (ny, nx) = (y as isize + DY[dir as usize], x as isize + DX[dir as usize]); 
    let in_bounds = 0 <= ny && ny < (r as isize) && 0 <= nx && nx < (c as isize);  
    return if !in_bounds { return None } else { Some((ny as usize, nx as usize)) }; 
}

fn bfs(input: &Vec<String>, (sy, sx): (usize, usize)) -> Vec<Vec<Option<usize>>> {
    let (r, c) = (input.len(), input[0].len());
    let mut dist = vec![vec![None; c]; r]; 
    let mut dq = VecDeque::<(usize, usize, usize)>::new(); 
    dq.push_back((sy, sx, 0)); 
    while !dq.is_empty() {
        let (y, x, d) = dq.pop_front().unwrap(); 
        if dist[y][x].is_some() {
            continue; 
        } 
        dist[y][x] = Some(d); 
        for dir in DIRECTION::VALUES {
            match get_next((r, c), (y, x), dir) {
                None => continue,
                Some((ny, nx)) => {
                    if dist[ny][nx].is_none() && input[ny].as_bytes()[nx] != b'#' {
                        dq.push_back((ny, nx, d+1));  
                    }
                }
            }
        }  
    }
    return dist; 
}

fn part1(input: &Vec<String>) -> u32 {
    const DISTANCE: usize = 64; 
    let (sy, sx) = get_start(input).unwrap(); 
    let dist = bfs(input, (sy, sx)); 
    let mut cnt = 0; 
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            match dist[i][j] {
                None => continue,
                Some(d) => {
                    if d <= DISTANCE && d%2 == DISTANCE%2 {
                        cnt += 1; 
                    }
                }
            }
        }
    }
    return cnt; 
}

fn part2(input: &Vec<String>) -> u64 {
    const DISTANCE: usize = 26501365; 
    let (r, c) = (input.len(), input[0].len()); 
    let (sy, sx) = get_start(input).unwrap();  
    let corners: [(usize, usize); 4] = [(0, 0), (r-1, 0), (0, c-1), (r-1, c-1)]; 
    let dist = Vec::<Vec<Vec<Option<usize>>>>::new(); 
    for corner in corners {
        dist.push(bfs(input, corner)); 
    }
    let mut cnt = 0; 
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..4 {
                for l in 0..4 {
                    let from = dist[k][sy][sx].unwrap(); 
                    let to = dist[l][sy][sx].unwrap(); 
                    let (mut m, mut pw) = (1, 1); 
                    while from + to + m*r <= DISTANCE {
                        
                    }
                }
            }
        }
    }
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