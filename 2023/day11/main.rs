use std::io::{self, BufRead}; 
use std::cmp::{min, max}; 

fn get_galaxies(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::<(usize, usize)>::new(); 
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((i, j)); 
            }
        }
    }
    return galaxies; 
}

fn process_rows(input: &Vec<String>) -> Vec<u64> {
    let mut rows = vec![0; input.len()+1]; 
    for (i, line) in input.iter().enumerate() {
        let mut empty = true; 
        for ch in line.chars() {
            empty = empty && ch == '.'; 
        }     
        rows[i+1] = rows[i]; 
        if empty {
            rows[i+1] += 1; 
        }
    }
    return rows; 
}

fn process_cols(input: &Vec<String>) -> Vec<u64> {
    let mut cols = vec![0; input[0].len()+1];
    for j in 0..input[0].len() {
        let mut empty = true; 
        for i in 0..input.len() {
            empty = empty && input[i].as_bytes()[j] == b'.';  
        } 
        cols[j+1] = cols[j]; 
        if empty {
            cols[j+1] += 1; 
        }  
    }
    return cols; 
}

fn part1(input: &Vec<String>) -> u64 {
    let galaxies = get_galaxies(input); 
    let rows = process_rows(input); 
    let cols = process_cols(input); 
    let mut total = 0; 
    for (y1, x1) in &galaxies {
        for (y2, x2) in &galaxies {
            let (yy1, yy2) = (min(*y1, *y2), max(*y1, *y2)); 
            let (xx1, xx2) = (min(*x1, *x2), max(*x1, *x2)); 
            let dist = (yy2-yy1) as u64 + (rows[yy2+1]-rows[yy1]) +
                (xx2-xx1) as u64 + (cols[xx2+1]-cols[xx1]); 
            total += dist; 
        }
    }
    return total / 2; 
}

fn part2(input: &Vec<String>) -> u64 {
    let galaxies = get_galaxies(input); 
    let rows = process_rows(input); 
    let cols = process_cols(input); 
    let mut total = 0; 
    for (y1, x1) in &galaxies {
        for (y2, x2) in &galaxies {
            let (yy1, yy2) = (min(*y1, *y2), max(*y1, *y2)); 
            let (xx1, xx2) = (min(*x1, *x2), max(*x1, *x2)); 
            let dist = (yy2-yy1) as u64 + 999999*(rows[yy2+1]-rows[yy1]) +
                (xx2-xx1) as u64 + 999999*(cols[xx2+1]-cols[xx1]); 
            total += dist; 
        }
    }
    return total / 2; 
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