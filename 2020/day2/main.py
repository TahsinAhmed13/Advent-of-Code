from sys import stdin

def parse(line): 
    tokens = line.split()
    idx = tokens[0].index('-')
    x = int(tokens[0][:idx])
    y = int(tokens[0][idx+1:])
    ch = tokens[1][:-1]
    s = tokens[2]
    return (x, y, ch, s)

def valid(passwd): 
    (x, y, ch, s) = passwd
    cnt = s.count(ch)
    return x <= cnt and cnt <= y

def part1(passwds): 
    return len(list(filter(valid, passwds)))

def valid2(passwd): 
    (x, y, ch, s) = passwd
    return (ch == s[x-1]) ^ (ch == s[y-1])

def part2(passwds): 
    return len(list(filter(valid2, passwds)))

passwds = [parse(line) for line in stdin.readlines()]
print(part1(passwds))
print(part2(passwds))
    
