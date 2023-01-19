from sys import stdin
from copy import deepcopy

dx = [-1, -1, -1, 0, 1, 1, 1, 0]
dy = [-1, 0, 1, 1, 1, 0, -1, -1]

def part1(grid): 
    cur = [list(row) for row in grid]
    while True: 
        nxt = deepcopy(cur)
        for i in range(0, len(cur)): 
            for j in range(0, len(cur[i])): 
                cnt = 0 
                for k in range(0, len(dx)): 
                    ni = i+dx[k]
                    nj = j+dy[k]
                    inBounds = 0 <= ni and ni < len(cur) and 0 <= nj and nj < len(cur[i])
                    cnt += inBounds and cur[ni][nj] == '#'
                if cur[i][j] == 'L' and cnt == 0: 
                    nxt[i][j] = '#'
                if cur[i][j] == '#' and cnt >= 4: 
                    nxt[i][j] = 'L'
        if cur == nxt: 
            break
        cur = nxt
    cnt = 0
    for row in cur: 
        cnt += row.count('#')
    return cnt

def in_bounds(grid, r, c): 
    return 0 <= r and r < len(grid) and 0 <= c and c < len(grid[0])

def part2(grid): 
    cur = [list(row) for row in grid]
    while True: 
        nxt = deepcopy(cur)
        for i in range(0, len(cur)): 
            for j in range(0, len(cur[i])): 
                if cur[i][j] == '.': 
                    continue
                cnt = 0
                for k in range(0, len(dx)): 
                    r, c = i+dx[k], j+dy[k]
                    while in_bounds(grid, r, c) and cur[r][c] == '.': 
                        r += dx[k]
                        c += dy[k]
                    cnt += in_bounds(grid, r, c) and cur[r][c] == '#'
                if cur[i][j] == 'L' and cnt == 0: 
                    nxt[i][j] = '#'
                if cur[i][j] == '#' and cnt >= 5: 
                    nxt[i][j] = 'L'
        if cur == nxt: 
            break
        cur = nxt
    cnt = 0
    for row in cur: 
        cnt += row.count('#')
    return cnt

grid = stdin.read().splitlines()
print(part1(grid))
print(part2(grid))
