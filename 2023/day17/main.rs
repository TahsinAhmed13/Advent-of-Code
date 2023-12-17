use std::io::{self, BufRead}; 
use std::collections::{BinaryHeap, HashSet}; 

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Direction { UP, RIGHT, DOWN, LEFT } 
const DY: [isize; 4] = [-1, 0, 1, 0]; 
const DX: [isize; 4] = [0, 1, 0, -1]; 

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::LEFT,
        Direction::RIGHT => Direction::UP,
        Direction::DOWN => Direction::RIGHT,
        Direction::LEFT => Direction::DOWN,  
    } 
}

fn turn_right(dir: Direction) ->  Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    } 
}

fn get_next((r, c): (usize, usize), (y, x, dir): (usize, usize, Direction)) -> Option<(usize, usize)> {
    let (ny, nx) = (y as isize + DY[dir as usize], x as isize + DX[dir as usize]); 
    let in_bounds = 0 <= ny && ny < r as isize && 0 <= nx && nx < c as isize; 
    if in_bounds {
        return Some(((ny as usize), (nx as usize))); 
    }
    return None; 
}

fn parse_grid(input: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; input[0].len()]; input.len()];  
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() { 
            grid[i][j] = ch.to_digit(10).unwrap(); 
        }
    }
    return grid; 
}

fn get_heat_loss(input: &Vec<String>, (lo, hi): (usize, usize)) -> u32 {
    let grid = parse_grid(input); 
    let (r, c) = (grid.len(), grid[0].len()); 
    let mut seen = HashSet::<(usize, usize, Direction, usize)>::new(); 
    let mut pq = BinaryHeap::<(i32, usize, usize, Direction, usize)>::new();  
    pq.push((0, 0, 0, Direction::RIGHT, 0)); 
    pq.push((0, 0, 0, Direction::DOWN, 0)); 
    while !pq.is_empty() {
        let (dist, y, x, dir, cnt) = pq.pop().unwrap(); 
        if seen.contains(&(y, x, dir, cnt)) {
            continue; 
        } 
        seen.insert((y, x, dir, cnt)); 
        if y == r-1 && x == c-1 && cnt >= lo {
            return (-dist) as u32; 
        }
        let (left, right) = (turn_left(dir), turn_right(dir)); 
        if cnt < hi {
            match get_next((r, c), (y, x, dir)) {
                None => (),
                Some((ny, nx)) => pq.push((dist-grid[ny][nx] as i32, ny, nx, dir, cnt+1)), 
            }
        }
        if cnt >= lo {
            match get_next((r, c), (y, x, left)) {
                None => (),
                Some((ny, nx)) => pq.push((dist-grid[ny][nx] as i32, ny, nx, left, 1)), 
            }
            match get_next((r, c), (y, x, right)) {
                None => (),
                Some((ny, nx)) => pq.push((dist-grid[ny][nx] as i32, ny, nx, right, 1)), 
            }
        } 
    }
    return u32::MAX; 
}

fn part1(input: &Vec<String>) -> u32 {
    return get_heat_loss(input, (0, 3)); 
}

fn part2(input: &Vec<String>) -> u32 {
    return get_heat_loss(input, (4, 10)); 
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