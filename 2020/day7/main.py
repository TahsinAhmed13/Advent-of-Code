from sys import stdin

def has_gold(adj, bag): 
    for (_, other) in adj[bag]: 
        if other == 'shiny gold' or has_gold(adj, other): 
            return True
    return False

def part1(adj): 
    return sum(map(lambda b: has_gold(adj, b), adj.keys()))

def count_bags(adj, bag): 
    res = 0
    for (cnt, other) in adj[bag]: 
        res += cnt+cnt*count_bags(adj, other)
    return res

def part2(adj): 
    return count_bags(adj, 'shiny gold')

adj = dict()
for line in stdin.read().splitlines(): 
    tokens = line.split()
    bag = tokens[0]+' '+tokens[1]
    adj[bag] = list()
    cur = 4
    while cur+3 < len(tokens): 
        cnt = int(tokens[cur])
        other = tokens[cur+1]+' '+tokens[cur+2]
        adj[bag].append((cnt, other))
        cur += 4
print(part1(adj)) 
print(part2(adj))
