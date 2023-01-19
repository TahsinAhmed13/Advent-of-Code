from sys import stdin

def part1(instrs): 
    dx = [1, 0, -1, 0]
    dy = [0, 1, 0, -1]
    x, y = 0, 0
    cur = 0 
    for instr in instrs: 
        if instr[0] == 'N': 
            y += int(instr[1:])
        elif instr[0] == 'S': 
            y -= int(instr[1:])
        elif instr[0] == 'E': 
            x += int(instr[1:])
        elif instr[0] == 'W': 
            x -= int(instr[1:])
        elif instr[0] == 'L': 
            cur = (cur+(int(instr[1:])//90))%4
        elif instr[0] == 'R': 
            cur = (cur-(int(instr[1:])//90))%4
        else:
            x += int(instr[1:])*dx[cur]
            y += int(instr[1:])*dy[cur]
    return abs(x)+abs(y)

def rot90(x, y): 
    return -y, x

def rot270(x, y): 
    return y, -x

def part2(instrs): 
    x, y = 0, 0
    wx, wy = 10, 1
    cur = 0 
    for instr in instrs: 
        if instr[0] == 'N': 
            wy += int(instr[1:])
        elif instr[0] == 'S': 
            wy -= int(instr[1:])
        elif instr[0] == 'E': 
            wx += int(instr[1:])
        elif instr[0] == 'W': 
            wx -= int(instr[1:])
        elif instr[0] == 'L': 
            for _ in range(0, int(instr[1:])//90): 
                wx, wy = rot90(wx, wy)
        elif instr[0] == 'R': 
            for _ in range(0, int(instr[1:])//90): 
                wx, wy = rot270(wx, wy)
        else: 
            x += wx*int(instr[1:])
            y += wy*int(instr[1:])
    return abs(x)+abs(y)

instrs = stdin.read().splitlines()
print(part1(instrs))
print(part2(instrs))
