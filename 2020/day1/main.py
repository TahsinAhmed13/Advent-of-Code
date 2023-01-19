from sys import stdin
from itertools import product

data = [int(num) for num in stdin.readlines()]

def part1(a):
    for (x, y) in product(a, repeat=2): 
        if x+y == 2020:
            return x*y

def part2(a): 
    for (x, y, z) in product(a, repeat=3): 
        if x+y+z == 2020: 
            return x*y*z

print(part1(data))
print(part2(data))
