from sys import stdin
from collections import deque
import itertools

def score(a, b): 
    res = 0 
    for i in range(0, len(a)): 
        res += a[i]*(len(a)-i)
    for i in range(0, len(b)):  
        res += b[i]*(len(b)-i)
    return res

def part1(a, b): 
    while a and b: 
        x, y = a.popleft(), b.popleft()
        if x > y: 
            a.append(x)
            a.append(y)
        else: 
            b.append(y)
            b.append(x)
    return score(a, b)
    
def play(a, b): 
    configs = set()
    while a and b and (tuple(a), tuple(b)) not in configs: 
        configs.add((tuple(a), tuple(b)))
        x, y = a.popleft(), b.popleft()
        if x <= len(a) and y <= len(b): 
            winner = play(deque(itertools.islice(a, 0, x)), deque(itertools.islice(b, 0, y)))
        else: 
            winner = x < y
        if winner == 0: 
            a.append(x)
            a.append(y)
        else: 
            b.append(y)
            b.append(x)
    return not a
            
def part2(a, b): 
    play(a, b)
    return score(a, b)

lines = stdin.read().splitlines()
a = deque([int(x) for x in lines[1:lines.index('')]])
b = deque([int(x) for x in lines[lines.index('')+2:]])
print(part1(a.copy(), b.copy()))
print(part2(a.copy(), b.copy()))
