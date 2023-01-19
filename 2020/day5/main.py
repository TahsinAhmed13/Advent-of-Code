from sys import stdin 

def seatid(seat): 
    row = 0
    for i in range(0, 7): 
        if seat[i] == 'B': 
            row += 1<<(6-i)
    col = 0
    for i in range(0, 3): 
        if seat[i+7] == 'R': 
            col += 1<<(2-i)
    return 8*row+col

def part1(seats): 
    return max(list(map(seatid, seats)))

def part2(seats): 
    ids = sorted(list(map(seatid, seats)))
    for i in range(0, len(ids)-1): 
        if(ids[i]+2 == ids[i+1]): 
            return ids[i]+1

seats = stdin.read().splitlines()
print(part1(seats))
print(part2(seats))
