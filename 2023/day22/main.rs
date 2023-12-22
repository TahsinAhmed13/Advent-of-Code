use std::io::{self, BufRead}; 
use std::cmp::{max}; 
use std::collections::{VecDeque}; 

type Point = (usize, usize, usize); 
type Brick = (Point, Point); 

fn parse_brick(line: &str) -> Brick {
    let tokens: Vec<&str> = line.split([',', '~']).collect(); 
    let x1 = tokens[0].parse::<usize>().unwrap(); 
    let y1 = tokens[1].parse::<usize>().unwrap();  
    let z1 = tokens[2].parse::<usize>().unwrap(); 
    let x2 = tokens[3].parse::<usize>().unwrap(); 
    let y2 = tokens[4].parse::<usize>().unwrap(); 
    let z2 = tokens[5].parse::<usize>().unwrap(); 
    return ((x1, y1, z1), (x2, y2, z2)); 
}

fn parse_bricks(input: &Vec<String>) -> Vec<Brick> {
    let mut bricks = Vec::<Brick>::new(); 
    for line in input {
        bricks.push(parse_brick(line)); 
    } 
    return bricks; 
}

fn find_supports(bricks: &mut Vec<Brick>) -> Vec<Vec<usize>> {
    bricks.sort_by(|a, b| a.0.2.cmp(&b.0.2));  
    let mut supports = Vec::<Vec<usize>>::new(); 
    for _ in 0..bricks.len() {
        supports.push(Vec::<usize>::new()); 
    }
    const SIZE: usize = 10; 
    let mut grid = [[0; SIZE]; SIZE]; 
    for k in 0..bricks.len() {
        let ((x1, y1, _), (x2, y2, _)) = bricks[k]; 
        let mut top = 0; 
        for i in x1..x2+1 {
            for j in y1..y2+1 {
                if grid[i][j] > 0 {
                    top = max(top, bricks[grid[i][j]-1].1.2); 
                }
            }
        }
        bricks[k].1.2 -= bricks[k].0.2 - top - 1; 
        bricks[k].0.2 -= bricks[k].0.2 - top - 1; 
        let mut used = vec![false; bricks.len()];  
        for i in x1..x2+1 {
            for j in y1..y2+1 {
                let val = grid[i][j]; 
                if val > 0 && !used[val-1] && bricks[val-1].1.2 == top {
                    supports[k].push(val-1); 
                    used[val-1] = true; 
                }
                grid[i][j] = k+1;  
            }
        }
    }
    return supports; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut bricks = parse_bricks(input); 
    let mut freq = vec![0; bricks.len()];  
    for support in find_supports(&mut bricks) {
        if support.len() == 1 {
            freq[support[0]] += 1; 
        }
    }
    let mut cnt = 0; 
    for x in freq {
        if x == 0 { cnt += 1 };   
    }
    return cnt; 
}

fn reverse_graph(adj: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut rev = Vec::<Vec<usize>>::new(); 
    for _ in 0..adj.len() {
        rev.push(Vec::<usize>::new()); 
    }     
    for (u, neighbors) in adj.iter().enumerate() {
        for v in neighbors {
            rev[*v].push(u); 
        }
    }  
    return rev; 
}

fn get_reactions(adj: &Vec<Vec<usize>>, start: usize) -> u32 {
    let rev = reverse_graph(adj);  
    let mut deg = vec![0; adj.len()]; 
    for i in 0..adj.len() {
        deg[i] = adj[i].len(); 
    }
    let mut cnt = 0; 
    let mut dq = VecDeque::<usize>::new();  
    dq.push_back(start); 
    while !dq.is_empty() {
        let u = dq.pop_front().unwrap(); 
        for v in &rev[u] {
            deg[*v] -= 1; 
            if deg[*v] == 0 {
                dq.push_back(*v); 
            } 
        }
        cnt += 1; 
    }  
    return cnt - 1; 
}

fn part2(input: &Vec<String>) -> u32 {
    let mut bricks = parse_bricks(input); 
    let adj = find_supports(&mut bricks); 
    let mut res = 0;  
    for i in 0..bricks.len() {
        res += get_reactions(&adj, i); 
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
