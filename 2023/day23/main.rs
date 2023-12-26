use std::io::{self, BufRead}; 
use std::cmp::{max}; 
use std::collections::{HashMap, HashSet}; 
type Point = (usize, usize); 

#[derive(Copy, Clone, Debug)]
enum DIRECTION { UP, RIGHT, DOWN, LEFT } 
impl DIRECTION {
    const VALUES: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]; 
}

const DY: [isize; 4] = [-1, 0, 1, 0]; 
const DX: [isize; 4] = [0, 1, 0, -1]; 
const SLOPES: &str = "^>v<"; 

fn get_next(input: &Vec<String>, (y, x): Point, dir: DIRECTION) -> Option<Point> {
    let (r, c) = (input.len(), input[0].len()); 
    let (ny, nx) = (y as isize + DY[dir as usize], x as isize + DX[dir as usize]); 
    let in_bounds = 0 <= ny && ny < (r as isize) && 0 <= nx && nx < (c as isize);  
    if !in_bounds {
        return None; 
    }
    let (ny, nx) = (ny as usize, nx as usize); 
    return if input[ny].as_bytes()[nx] == b'#' { None } else { Some((ny, nx)) }; 
}

fn is_checkpoint(input: &Vec<String>, (y, x): Point) -> bool {
    let mut cnt = 0;  
    for dir in DIRECTION::VALUES {
        match get_next(input, (y, x), dir) {
            None => continue,
            Some(_) => cnt += 1 
        }
    }
    return y == 0 || y == input.len()-1 || cnt > 2; 
}

fn longest_path(adj: &HashMap<Point, Vec<(Point, usize)>>, start: Point, end: Point) -> u32 {
    let mut max_dist = 0; 
    let mut path = HashSet::<Point>::new(); 
    let mut st = vec![]; 
    st.push((start, 0, false)); 
    while !st.is_empty() {
        let (point, dist, seen) = st.pop().unwrap(); 
        if seen {
            path.remove(&point); 
            continue; 
        }
        if path.contains(&point) {
            continue; 
        }
        if point == end {
            max_dist = max(max_dist, dist); 
        }
        path.insert(point); 
        st.push((point, dist, true)); 
        match adj.get(&point) {
            None => continue,
            Some(neighbors) => {
                for (next, to) in neighbors {
                    st.push((*next, dist + to, false)); 
                }
            }
        }
    }
    return max_dist as u32; 
}

fn part1(input: &Vec<String>) -> u32 {
    let (r, c) = (input.len(), input[0].len()); 
    let mut adj = HashMap::<Point, Vec<(Point, usize)>>::new();  
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '.' && is_checkpoint(input, (i, j)) {
                let mut neighbors = vec![]; 
                let mut seen = HashSet::<Point>::new(); 
                let mut st = vec![]; 
                st.push(((i, j), 0)); 
                while !st.is_empty() {
                    let (point, dist) = st.pop().unwrap(); 
                    if seen.contains(&point) {
                        continue; 
                    } 
                    seen.insert(point); 
                    if is_checkpoint(input, point) && dist > 0 {
                        neighbors.push((point, dist)); 
                        continue; 
                    }
                    let (y, x) = point; 
                    let ch = input[y].chars().nth(x).unwrap(); 
                    match SLOPES.find(ch) {
                        None => {
                            for dir in DIRECTION::VALUES {
                                match get_next(input, point, dir) {
                                    None => continue,
                                    Some(next) => st.push((next, dist+1))
                                }
                            }
                        },
                        Some(idx) => {
                            match get_next(input, point, DIRECTION::VALUES[idx]) {
                                None => continue,
                                Some(next) => st.push((next, dist+1))
                            }
                        }
                    }
                }
                adj.insert((i, j), neighbors); 
            }
        }
    } 
    return longest_path(&adj, (0, 1), (r-1, c-2)); 
}

fn part2(input: &Vec<String>) -> u32 {
    let (r, c) = (input.len(), input[0].len()); 
    let mut adj = HashMap::<Point, Vec<(Point, usize)>>::new();  
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '.' && is_checkpoint(input, (i, j)) {
                let mut neighbors = vec![]; 
                let mut seen = HashSet::<Point>::new(); 
                let mut st = vec![]; 
                st.push(((i, j), 0)); 
                while !st.is_empty() {
                    let (point, dist) = st.pop().unwrap(); 
                    if seen.contains(&point) {
                        continue; 
                    } 
                    seen.insert(point); 
                    if is_checkpoint(input, point) && dist > 0 {
                        neighbors.push((point, dist)); 
                        continue; 
                    }
                    for dir in DIRECTION::VALUES {
                        match get_next(input, point, dir) {
                            None => continue,
                            Some(next) => st.push((next, dist+1))
                        }
                    }
                }
                adj.insert((i, j), neighbors); 
            }
        }
    } 
    return longest_path(&adj, (0, 1), (r-1, c-2)); 
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