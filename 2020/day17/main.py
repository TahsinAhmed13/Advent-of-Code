from sys import stdin
from itertools import product

def part1(grid): 
    active = {(x, y, 0) for y in range(0, len(grid)) for x in range(0, len(grid[y])) if grid[y][x] == '#'}
    for _ in range(0, 6): 
        nxt = set()
        freq = dict()
        for x, y, z in active: 
            cnt = 0
            for dx, dy, dz in product(range(-1, 2), repeat=3): 
                if dx == 0 and dy == 0 and dz == 0: 
                    continue
                cnt += (x+dx, y+dy, z+dz) in active
                if (x+dx, y+dy, z+dz) in freq: 
                    freq[(x+dx, y+dy, z+dz)] += 1
                else: 
                    freq[(x+dx, y+dy, z+dz)] = 1
            if cnt == 2 or cnt == 3: 
                nxt.add((x, y, z))
        for k, v in freq.items(): 
            if v == 3: 
                nxt.add(k)
        active = nxt
    return len(active)

def part2(grid): 
    active = {(x, y, 0, 0) for y in range(0, len(grid)) for x in range(0, len(grid[y])) if grid[y][x] == '#'}
    for _ in range(0, 6): 
        nxt = set()
        freq = dict()
        for x, y, z, w in active: 
            cnt = 0
            for dx, dy, dz, dw in product(range(-1, 2), repeat=4): 
                if dx == 0 and dy == 0 and dz == 0 and dw == 0: 
                    continue
                cnt += (x+dx, y+dy, z+dz, w+dw) in active
                if (x+dx, y+dy, z+dz, w+dw) in freq: 
                    freq[(x+dx, y+dy, z+dz, w+dw)] += 1
                else: 
                    freq[(x+dx, y+dy, z+dz, w+dw)] = 1
            if cnt == 2 or cnt == 3: 
                nxt.add((x, y, z, w))
        for k, v in freq.items(): 
            if v == 3: 
                nxt.add(k)
        active = nxt
    return len(active)

grid = stdin.read().splitlines()
print(part1(grid))
print(part2(grid))
