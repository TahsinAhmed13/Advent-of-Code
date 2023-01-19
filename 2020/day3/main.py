from sys import stdin
from math import prod

def trees(grid, slope): 
    res = 0 
    (dy, dx) = slope
    y, x = 0, 0
    while y+dy < len(grid): 
        y += dy 
        x = (x+dx)%len(grid[0])
        res += grid[y][x] == '#'
    return res

def part1(grid): 
    return trees(grid, (1, 3))

def part2(grid): 
    slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    return prod([trees(grid, slope) for slope in slopes])

data = stdin.read().splitlines()
print(part1(data))
print(part2(data))
        
