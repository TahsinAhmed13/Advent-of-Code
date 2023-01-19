from sys import stdin 

BASE = 7
MOD = 20201227

def bexp(x, y, m): 
    res = 1
    while y > 0: 
        if y%2: 
            res *= x
            res %= m
        x *= x
        x %= m
        y //= 2 
    return res

def dlog(x, y, m): 
    k = 0
    val = 1
    while val != y: 
        val *= x
        val %= m
        k += 1
    return k

def part1(x, y): 
    a = dlog(BASE, x, MOD)
    b = dlog(BASE, y, MOD)
    return bexp(BASE, a*b, MOD)

x = int(stdin.readline())
y = int(stdin.readline())
print(part1(x, y))
