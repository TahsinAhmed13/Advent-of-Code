use std::io::{self, BufRead}; 
use std::collections::HashMap; 

fn tilt_north(input: &mut Vec<Vec<char>>) {
    for j in 0..input[0].len() {
        for i in 0..input.len() {
            if input[i][j] != 'O' {
                continue; 
            }
            let mut k = i; 
            while k > 0 && input[k-1][j] == '.' {
                k -= 1; 
            } 
            input[i][j] = '.';  
            input[k][j] = 'O'; 
        }
    }        
}

fn tilt_west(input: &mut Vec<Vec<char>>) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != 'O' {
                continue; 
            }
            let mut k = j; 
            while k > 0 && input[i][k-1] == '.' {
                k -= 1; 
            } 
            input[i][j] = '.';  
            input[i][k] = 'O'; 
        }
    }        
}

fn tilt_south(input: &mut Vec<Vec<char>>) {
    for j in 0..input[0].len() {
        for i in (0..input.len()).rev() {
            if input[i][j] != 'O' {
                continue; 
            }
            let mut k = i; 
            while k+1 < input.len() && input[k+1][j] == '.' {
                k += 1; 
            }
            input[i][j] = '.'; 
            input[k][j] = 'O'; 
        }
    }
}

fn tilt_east(input: &mut Vec<Vec<char>>) {
    for i in 0..input.len() {
        for j in (0..input[i].len()).rev() {
            if input[i][j] != 'O' {
                continue; 
            }
            let mut k = j; 
            while k+1 < input[i].len() && input[i][k+1] == '.' {
                k += 1; 
            } 
            input[i][j] = '.'; 
            input[i][k] = 'O'; 
        } 
    }
}

fn get_load(input: &Vec<Vec<char>>) -> u32 {
    let mut res = 0; 
    for j in 0..input[0].len() {
        for i in (0..input.len()).rev() {
            if input[i][j] == 'O' {
                res += (input.len() - i) as u32; 
            }
        }
    }
    return res; 
}

fn grid_to_str(grid: &Vec<Vec<char>>) -> String {
    let mut res = String::from(""); 
    for line in grid { 
        res += &line.iter().collect::<String>(); 
    }
    return res; 
}

fn part1(input: &Vec<String>) -> u32 {
    let mut grid = Vec::<Vec<char>>::new(); 
    for line in input{
        grid.push(line.chars().collect()); 
    } 
    tilt_north(&mut grid); 
    return get_load(&grid); 
}

fn part2(input: &Vec<String>) -> u32 {
    const ITER: usize = 1000000000; 
    let mut grid = Vec::<Vec<char>>::new(); 
    for line in input{
        grid.push(line.chars().collect()); 
    } 

    let mut cycles = HashMap::<String, usize>::new(); 
    let mut loads = Vec::<u32>::new(); 
    let mut i: usize = 0; 
    loop {
        let key = grid_to_str(&grid); 
        if cycles.contains_key(&key) {
            break; 
        }     
        cycles.insert(key, i); 
        loads.push(get_load(&mut grid)); 

        tilt_north(&mut grid); 
        tilt_west(&mut grid); 
        tilt_south(&mut grid); 
        tilt_east(&mut grid); 

        i += 1;   
    }
    
    let start = cycles.get(&grid_to_str(&grid)).unwrap();  
    let len = i - start; 
    return loads[start + (ITER - i) % len];  
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