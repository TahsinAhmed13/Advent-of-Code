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

fn count_plots(input: &Vec<String>, max_dist: usize) -> u64 {
    let (r, c) = (input.len(), input[0].len()); 
    let (sy, sx) = get_start(input).unwrap();
    let dist_start = bfs(input, (sy, sx)); 

    let corners = [(0, 0), (0, c-1), (r-1, c-1), (r-1, 0)]; 
    let mut dist_corners = vec![]; 
    for i in 0..corners.len() {
        dist_corners.push(bfs(input, corners[i])); 
    }

    let sides = [(sy, 0), (0, sx), (sy, c-1), (r-1, sx)]; 
    let mut dist_sides = vec![]; 
    for i in 0..sides.len() {
        dist_sides.push(bfs(input, sides[i])); 
    }

    let max_steps = max_dist / r;  
    let (mut evens, mut odds) = (vec![0; max_steps+1], vec![0; max_steps+1]); 
    evens[0] = 1; 
    for i in 1..max_steps+1 {
        evens[i] = evens[i-1]; 
        odds[i] = odds[i-1]; 
        if i % 2 == 0 {
            evens[i] += i as u64 + 1; 
        } else {
            odds[i] += i as u64 + 1; 
        }
    } 

    let mut cnt = 0; 
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            match dist_start[i][j] {
                None => continue,
                Some(d) => if d <= max_dist && (max_dist-d) % 2 == 0 { cnt += 1 },
            }

            for k in 0..corners.len() {
                let opp = (k+2) % corners.len(); 
                let to = dist_corners[k][sy][sx].unwrap() + 2; 
                match dist_corners[opp][i][j] {
                    None => continue,
                    Some(from) => {
                        if to + from <= max_dist {
                            let steps = (max_dist - to - from) / r; 
                            if (to + from) % 2 == max_dist % 2 {
                                cnt += evens[steps]; 
                            } 
                            if (to + from + r) % 2 == max_dist % 2 {
                                cnt += odds[steps]; 
                            }
                        }
                    }
                }
            }

            for k in 0..sides.len() {
                let opp = (k+2) % sides.len(); 
                let to = dist_sides[k][sy][sx].unwrap() + 1; 
                match dist_sides[opp][i][j] {
                    None => continue,
                    Some(from) => {
                        if to + from <= max_dist {
                            let steps = (max_dist - to - from) / r; 
                            if (to + from) % 2 == max_dist % 2 {
                                cnt += (steps as u64 / 2) + 1; 
                            }
                            if (to + from + r) % 2 == max_dist % 2 {
                                cnt += (steps as u64 + 1) / 2; 
                            }
                        }  
                    }
                }
            }
        }
    }
    return cnt; 
}

fn part1(input: &Vec<String>) -> u64 {
    const DISTANCE: usize = 64; 
    return count_plots(input, DISTANCE); 
}

fn part2(input: &Vec<String>) -> u64 {
    const DISTANCE: usize = 26501365; 
    return count_plots(input, DISTANCE);  
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