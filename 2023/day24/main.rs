use std::io::{self, BufRead}; 
type Point = (f64, f64, f64); 

fn parse_hailstone(line: &str) -> (Point, Point) {
    let tokens: Vec<&str> = line.split_whitespace().collect(); 
    let px = tokens[0][..tokens[0].len()-1].parse::<f64>().unwrap();  
    let py = tokens[1][..tokens[1].len()-1].parse::<f64>().unwrap();  
    let pz = tokens[2].parse::<f64>().unwrap();  
    let vx = tokens[4][..tokens[4].len()-1].parse::<f64>().unwrap();  
    let vy = tokens[5][..tokens[5].len()-1].parse::<f64>().unwrap();  
    let vz = tokens[6].parse::<f64>().unwrap();  
    return ((px, py, pz), (vx, vy, vz)); 
}

fn get_intersection((p1, v1): (Point, Point), (p2, v2): (Point, Point)) -> Option<(f64, f64)> {
    const EPS: f64 = 1e-6; 
    let num = v1.0*v2.0*(p1.1-p2.1)-v2.0*v1.1*p1.0+v1.0*v2.1*p2.0; 
    let dom = v1.0*v2.1 - v2.0*v1.1; 
    if dom.abs() <= EPS {
        return None; 
    }
    let x = num / dom; 
    let t1 = (x-p1.0)/v1.0; 
    let t2 = (x-p2.0)/v2.0; 
    let y = p1.1+v1.1*t1; 
    return if t1 >= 0. && t2 >= 0. { Some((x, y)) } else { None }; 
}

fn part1(input: &Vec<String>) -> u32 {
    const MIN: f64 = 200000000000000.; 
    const MAX: f64 = 400000000000000.; 
    let mut stones = vec![]; 
    for line in input {
        stones.push(parse_hailstone(&line)); 
    }
    let mut cnt = 0; 
    for i in 0..stones.len() {
        for j in i+1..stones.len() {
            match get_intersection(stones[i], stones[j]) {
                None => continue,
                Some((x, y)) => {
                    if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                        cnt += 1; 
                    }
                }
            }
        }    
    }
    return cnt; 
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