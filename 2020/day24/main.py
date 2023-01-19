from sys import stdin

dirs = ['e', 'w', 'se', 'nw', 'sw', 'ne']
dx = [1, -1, 1, -1, 0, 0]
dy = [-1, 1, 0, 0, 1, -1]
dz = [0, 0, 1, -1, 1, -1]

def get_coords(tile): 
    x, y, z = 0, 0, 0
    cur = 0
    while cur < len(tile): 
        for i in range(0, len(dirs)): 
            if tile[cur:cur+len(dirs[i])] == dirs[i]: 
                x += dx[i]
                y += dy[i]
                z += dz[i]
                cur += len(dirs[i])
                break
    return x, y, z
   
def get_blacks(tiles): 
    flips = dict()
    for t in tiles: 
        coords = get_coords(t)
        flips[coords] = flips[coords]+1 if coords in flips else 1
    blacks = list()
    for k, v in flips.items(): 
        if v%2: 
            blacks.append(k)
    return blacks

def part1(tiles): 
    return len(get_blacks(tiles))

def part2(tiles): 
    cur = set(get_blacks(tiles))
    for _ in range(0, 100): 
        blacks, whites = dict(), dict()
        for (x, y, z) in cur: 
            blacks[(x, y, z)] = 0
            for i in range(0, len(dx)): 
                if (x+dx[i], y+dy[i], z+dz[i]) in cur: 
                    blacks[(x, y, z)] += 1
                elif (x+dx[i], y+dy[i], z+dz[i]) in whites: 
                    whites[(x+dx[i], y+dy[i], z+dz[i])] += 1
                else: 
                    whites[(x+dx[i], y+dy[i], z+dz[i])] = 1
        cur.clear()
        for k, v in blacks.items(): 
            if v == 1 or v == 2: 
                cur.add(k)
        for k, v in whites.items(): 
            if v == 2: 
                cur.add(k)
    return len(cur)

tiles = stdin.read().splitlines()
print(part1(tiles))
print(part2(tiles))
