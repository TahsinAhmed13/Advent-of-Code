use std::io::{self, BufRead}; 
use std::collections::{HashMap, HashSet, VecDeque}; 

pub struct Graph {
    adj: HashMap<String, HashSet<String>>
}

impl Graph {
    fn new() -> Self {
        let adj = HashMap::<String, HashSet<String>>::new(); 
        Self { adj }
    }

    fn from_edges(edges: &Vec<(String, String)>) -> Self {
        let mut graph = Self::new(); 
        for (u, v) in edges {
            graph.add_edge((&u, &v)); 
        }
        graph
    }

    fn len(&self) -> usize {
        self.adj.len()
    }

    fn add_edge(&mut self, (u, v): (&str, &str)) {
        match self.adj.get_mut(u) {
            Some(neighbors) => { neighbors.insert(v.to_string()); },
            None => {
                let mut neighbors = HashSet::<String>::new(); 
                neighbors.insert(v.to_string()); 
                self.adj.insert(u.to_string(), neighbors);  
            }
        }
        match self.adj.get_mut(v) {
            Some(neighbors) => { neighbors.insert(u.to_string()); },
            None => {
                let mut neighbors = HashSet::<String>::new();  
                neighbors.insert(u.to_string()); 
                self.adj.insert(v.to_string(), neighbors);  
            }
        }
    }

    fn remove_edge(&mut self, (u, v): (&str, &str)) {
        match self.adj.get_mut(u) {
            None => (),
            Some(neighbors) => { neighbors.remove(v); },
        }
        if self.neighbors(u).is_empty() {
            self.adj.remove(u);   
        }
        match self.adj.get_mut(v) {
            None => (), 
            Some(neighbors) => { neighbors.remove(u); }
        }
        if self.neighbors(v).is_empty() {
            self.adj.remove(v); 
        }
    }

    fn neighbors(&self, node: &str) -> Vec<&String> {
        match self.adj.get(node) {
            None => vec![],
            Some(neighbors) => neighbors.into_iter().collect()
        }
    }
}

fn parse_edges(input: &Vec<String>) -> Vec<(String, String)> {
    let mut edges = vec![]; 
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect(); 
        let src = &tokens[0][..tokens[0].len()-1]; 
        for dest in &tokens[1..] {
            edges.push((src.to_string(), dest.to_string())); 
        }
    }
    return edges; 
}

fn get_nodes(edges: &Vec<(String, String)>) -> Vec<String> {
    let mut nodes = HashSet::<String>::new(); 
    for (u, v) in edges {
        nodes.insert(u.clone()); 
        nodes.insert(v.clone()); 
    }
    return nodes.into_iter().collect(); 
}

fn dfs(graph: &Graph, src: &str) -> usize {
    let mut vis = HashSet::<String>::new(); 
    let mut st = vec![]; 
    st.push(src); 
    while !st.is_empty() {
        let node = st.pop().unwrap(); 
        if vis.contains(node) {
            continue; 
        }
        vis.insert(node.to_string()); 
        for to in graph.neighbors(node) {
            st.push(to); 
        }
    }     
    return vis.len(); 
}

fn bfs(graph: &Graph, src: &str, dest: &str) -> HashMap::<String, String> {
    let mut par = HashMap::<String, String>::new(); 
    let mut dq = VecDeque::<&str>::new();  
    dq.push_back(src); 
    while !dq.is_empty() {
        let node = dq.pop_front().unwrap();  
        if node == dest {
            break; 
        }
        for to in graph.neighbors(node) {
            if to != src && !par.contains_key(to) { 
                par.insert(to.to_string(), node.to_string()); 
                dq.push_back(to); 
            }  
        }
    }    
    return par; 
}

fn has_cut(graph: &mut Graph, src: &str, dest: &str, k: usize) -> bool {
    let mut flow = 0; 
    while flow <= k {
        let par = bfs(graph, src, dest); 
        if !par.contains_key(dest) {
            break; 
        }
        let mut cur = dest; 
        while cur != src {
            let next = par.get(cur).unwrap(); 
            graph.remove_edge((next, cur)); 
            cur = next; 
        }
        flow += 1; 
    }
    return flow <= k; 
}

fn part1(input: &Vec<String>) -> u32 {
    const CUT_SIZE: usize = 3; 
    let edges = parse_edges(input); 
    let nodes = get_nodes(&edges); 
    let mut ans = 0; 
    'outer:
    for (i, src) in nodes.iter().enumerate() {
        for dest in &nodes[i+1..] {
            let mut graph = Graph::from_edges(&edges);  
            if has_cut(&mut graph, src, dest, CUT_SIZE) {
                let size = dfs(&graph, src); 
                ans = size * (graph.len() - size); 
                break 'outer;        
            }    
        }
    }
    return ans as u32; 
}

fn main() {
    let mut input = Vec::new(); 
    let reader = io::stdin().lock(); 
    for line in reader.lines() {
        input.push(line.unwrap()); 
    }    
    println!("{}", part1(&input)); 
}