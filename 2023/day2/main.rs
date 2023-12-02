use std::io::{self, BufRead}; 

fn get_possible(game: &str, red: u32, green: u32, blue: u32) -> u32 {
    let tokens: Vec<&str> = game.split(' ').collect();  
    let id: u32 = tokens[1][..tokens[1].len()-1].parse().unwrap(); 
    for i in (2..tokens.len()).step_by(2) {
        let amt: u32 = tokens[i].parse().unwrap(); 
        let last = tokens[i+1].chars().last().unwrap(); 
        let color = if last == ',' || last == ';' { &tokens[i+1][..tokens[i+1].len()-1] } else { &tokens[i+1] }; 
        if color == "red" && amt > red {
            return 0; 
        } 
        if color == "green" && amt > green {
            return 0; 
        }   
        if color == "blue" && amt > blue {
            return 0; 
        } 
    }
    return id; 
}

fn part1(games: &Vec<String>) -> u32 {
    let (red, green, blue) = (12, 13, 14); 
    let mut total = 0;  
    for game in games {
        total += get_possible(&game, red, green, blue); 
    }
    return total; 
}

fn get_power(game: &str) -> u32 {
    let (mut red, mut green, mut blue) = (0, 0, 0);     
    let tokens: Vec<&str> = game.split(' ').collect();  
    for i in (2..tokens.len()).step_by(2) {
        let amt: u32 = tokens[i].parse().unwrap(); 
        let last = tokens[i+1].chars().last().unwrap(); 
        let color = if last == ',' || last == ';' { &tokens[i+1][..tokens[i+1].len()-1] } else { &tokens[i+1] }; 
        if color == "red" && amt > red {
            red = amt; 
        } 
        if color == "green" && amt > green {
            green = amt; 
        }   
        if color == "blue" && amt > blue {
            blue = amt; 
        } 
    }
    return red * green * blue; 
}

fn part2(games: &Vec<String>) -> u32 {
    let mut total = 0;  
    for game in games {
        total += get_power(&game); 
    }
    return total; 
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