use std::{io::{self, BufRead}, str::FromStr}; 

const EPS: f64 = 1e-6; 

#[derive(Copy, Clone, Debug)]
pub struct Point { x: f64, y: f64, _z: f64 }
#[derive(Copy, Clone, Debug)]
pub struct Ray { o: Point, v: Point }

impl Point {
    fn new(x: f64, y: f64, _z: f64) -> Self {
        Self { x, y, _z }
    }
}

impl Ray {
    fn new(o: Point, v: Point) -> Self {
        Self { o, v }
    }
}

fn parse_ray_offset(line: &str) -> Ray {
    let tokens: Vec<&str> = line.split_whitespace().collect(); 
    let px = f64::from_str(&tokens[0][..tokens[0].len()-1]).unwrap(); 
    let py = f64::from_str(&tokens[1][..tokens[1].len()-1]).unwrap(); 
    let pz = f64::from_str(&tokens[2]).unwrap(); 
    let vx = f64::from_str(&tokens[4][..tokens[4].len()-1]).unwrap(); 
    let vy = f64::from_str(&tokens[5][..tokens[5].len()-1]).unwrap(); 
    let vz = f64::from_str(&tokens[6]).unwrap(); 
    Ray::new(Point::new(px, py, pz), Point::new(vx, vy, vz)) 
}

fn det2d(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a*d - b*c
}

fn get_time(from: f64, to: f64, vel: f64) -> Option<f64> {
    let diff = to - from; 
    if diff.abs() > EPS && vel.abs() <= EPS {
        return None; 
    }
    Some(if diff.abs() <= EPS { 0. } else { diff / vel })
}

fn intersect2d(r1: &Ray, r2: &Ray) -> Option<Point> {
    let c1 = det2d(r1.o.x, r1.v.x, r1.o.y, r1.v.y); 
    let c2 = det2d(r2.o.x, r2.v.x, r2.o.y, r2.v.y); 
    let d = det2d(r1.v.y, -r1.v.x, r2.v.y, -r2.v.x); 
    if d.abs() <= EPS {
        return None; 
    }
    let x = det2d(c1, -r1.v.x, c2, -r2.v.x) / d; 
    let y = det2d(r1.v.y, c1, r2.v.y, c2) / d; 
    let t1 = get_time(r1.o.x, x, r1.v.x); 
    let t2 = get_time(r2.o.x, x, r2.v.x); 
    if t1.is_none() || t2.is_none() {
        return None; 
    }
    if t1.unwrap() >= 0. && t2.unwrap() >= 0. { Some(Point::new(x, y, 0.)) } else { None } 
}

fn part1(input: &Vec<String>) -> u32 {
    const MIN: f64 = 200000000000000.; 
    const MAX: f64 = 400000000000000.; 
    let rays: Vec<Ray> = input
        .into_iter()
        .map(|line| parse_ray_offset(&line))
        .collect(); 
    let mut cnt = 0; 
    for (i, r1) in rays.iter().enumerate() {
        for r2 in &rays[i+1..] {
            match intersect2d(r1, r2) {
                None => continue,
                Some(p) => {
                    if MIN <= p.x && p.x <= MAX && MIN <= p.y && p.y <= MAX {
                        cnt += 1; 
                    }
                }
            }
        }    
    }
    return cnt; 
}

fn main() {
    let mut input = Vec::new(); 
    let reader = io::stdin().lock(); 
    for line in reader.lines() {
        input.push(line.unwrap()); 
    }    
    println!("{}", part1(&input)); 
}