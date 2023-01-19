from sys import stdin

def part1(code): 
    vis = [False] * len(code)
    res = 0 
    cur = 0
    while not vis[cur]: 
        vis[cur] = True
        if code[cur][0] == 'acc': 
            res += code[cur][1]
        cur += code[cur][1] if code[cur][0] == 'jmp' else 1
    return res

def terminates(code, start): 
    vis = [False] * len(code)
    cur = start
    while cur < len(code) and not vis[cur]: 
        vis[cur] = True
        cur += code[cur][1] if code[cur][0] == 'jmp' else 1
    return cur == len(code)        

def fix(code, start): 
    if code[start][0] == 'jmp': 
        code[start] = ('nop', code[start][1])
        if terminates(code, start): 
            return
        code[start] = ('jmp', code[start][1])
    elif code[start][0] == 'nop': 
        code[start] = ('jmp', code[start][1])
        if terminates(code, start): 
            return
        code[start] = ('nop', code[start][1])
    if code[start][0] == 'jmp': 
        fix(code, start+code[start][1])
    else: 
        fix(code, start+1)

def part2(code): 
    fix(code, 0)
    res = 0
    cur = 0
    while cur < len(code): 
        if code[cur][0] == 'acc': 
            res += code[cur][1]
        cur += code[cur][1] if code[cur][0] == 'jmp' else 1
    return res
    
code = [(line[:3], int(line[4:])) for line in stdin.read().splitlines()]
print(part1(code))
print(part2(code))
