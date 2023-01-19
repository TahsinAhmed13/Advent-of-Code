from sys import stdin 

def part1(time, ids): 
    wait = (ids[0]-time%ids[0])%ids[0] 
    idx = 0 
    for i in range(1, len(ids)): 
        if ids[i] == 0: 
            continue
        val = (ids[i]-time%ids[i])%ids[i]
        if val < wait: 
            wait = val 
            idx = i
    return wait*ids[idx]

def bexp(x, y, m): 
    res = 1
    while y > 0: 
        if y%2 == 1: 
            res *= x
            res %= m
        x *= x
        x %= m
        y //= 2
    return res

def inverse(x, m): 
    return bexp(x, m-2, m)

def part2(ids): 
    M = 1
    for x in ids: 
        if x: 
            M *= x
    res = 0
    for i in range(0, len(ids)): 
        if ids[i]: 
            a = (-i)%ids[i]
            b = M//ids[i]
            res += (a*b*inverse(b, ids[i]))%M; 
            res %= M
    return res

time = int(stdin.readline())
ids = [int(num) if num != 'x' else 0 for num in stdin.readline().split(',')]
print(part1(time, ids))
print(part2(ids))
