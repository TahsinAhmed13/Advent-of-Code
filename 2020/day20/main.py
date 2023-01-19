from math import *
from sys import stdin

def get_borders(tile): 
    left = ''.join([line[0] for line in reversed(tile)])
    top = tile[0]
    right = ''.join([line[-1] for line in reversed(tile)])
    bottom = tile[-1]
    return left, top, right, bottom

def matches(a, b): 
    for x, y in zip(a, b): 
        if x != ' ' and x != y: 
            return False
    return True

def flip(tile): 
    return list(reversed(tile))

def rot90(tile): 
    return list([''.join([line[j] for line in reversed(tile)]) for j in range(0, len(tile[0]))])

def solve(tiles, puzzle, left, top, used, piece): 
    if piece >= len(tiles): 
        return True
    tile_width = len(tiles[0])
    puzzle_width = len(puzzle)
    row = piece // puzzle_width
    col = piece % puzzle_width
    left_start = (puzzle_width-row-1) * tile_width
    left_end = (puzzle_width-row) * tile_width
    top_start = col * tile_width
    top_end = (col+1) * tile_width
    left_edge = left[left_start:left_end]
    top_edge = top[top_start:top_end]
    for i in range(0, len(tiles)): 
        if used[i]: 
            continue
        for _ in range(0, 4): 
            borders = get_borders(tiles[i])
            if matches(left_edge, borders[0]) and matches(top_edge, borders[1]): 
                n_left = left[:left_start] + borders[2] + left[left_end:]
                n_top = top[:top_start] + borders[3] + top[top_end:]
                puzzle[row][col] = i 
                used[i] = True
                if solve(tiles, puzzle, n_left, n_top, used, piece+1): 
                    return True
                used[i] = False
            tiles[i] = flip(tiles[i])
            borders = get_borders(tiles[i])
            if matches(left_edge, borders[0]) and matches(top_edge, borders[1]): 
                n_left = left[:left_start] + borders[2] + left[left_end:]
                n_top = top[:top_start] + borders[3] + top[top_end:]
                puzzle[row][col] = i 
                used[i] = True
                if solve(tiles, puzzle, n_left, n_top, used, piece+1): 
                    return True
                used[i] = False
            tiles[i] = rot90(flip(tiles[i]))
    return False

def part1(ids, puzzle): 
    w = ids[puzzle[0][0]]
    x = ids[puzzle[0][-1]]
    y = ids[puzzle[-1][0]]
    z = ids[puzzle[-1][-1]]
    return w*x*y*z

def assemble(tiles, puzzle): 
    img = list()
    for row in puzzle: 
        for i in range(1, len(tiles[0])-1): 
            img.append(''.join([tiles[x][i][1:-1] for x in row]))
    return img

MONSTER = ['                  # ', '#    ##    ##    ###', ' #  #  #  #  #  #']

def get_monsters(img): 
    res = 0
    for i in range(0, len(img)-len(MONSTER)+1): 
        for j in range(0, len(img[i])-len(MONSTER[0])+1): 
            ok = True
            for k in range(0, len(MONSTER)): 
                ok = ok and matches(MONSTER[k], img[i+k][j:j+len(MONSTER[k])])
            res += ok
    return res

def part2(img): 
    monsters = 0 
    for _ in range(0, 4):  
        monsters += get_monsters(img)+get_monsters(flip(img))
        img = rot90(img)
    cnt = monsters*sum([line.count('#') for line in MONSTER]) 
    return sum([line.count('#') for line in img]) - cnt
    
lines = stdin.read().splitlines()
ids = list()
tiles = list()
borders = list()
last = 0
for i in range(0, len(lines)): 
    if not lines[i]: 
        ids.append(int(lines[last].split()[1][:-1]))
        tiles.append(lines[last+1:i])
        last = i+1
ids.append(int(lines[last].split()[1][:-1]))
tiles.append(lines[last+1:])

tile_width = len(tiles[0])
puzzle_width = int(sqrt(len(tiles)))
puzzle = list([[0] * puzzle_width for _ in range(0, puzzle_width)]) 
left = ' ' * tile_width * puzzle_width
top = ' ' * tile_width * puzzle_width
used = [False] * len(tiles)
solve(tiles, puzzle, left, top, used, 0)
img = assemble(tiles, puzzle)

print(part1(ids, puzzle))
print(part2(img))
