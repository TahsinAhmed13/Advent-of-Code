from sys import stdin

class Node:     
    def __init__(self, val, nxt = None): 
        self.val = val
        self.nxt = nxt

def build_list(cups): 
    mx = max(cups)
    refs = list()
    for i in range(0, mx+1): 
        refs.append(Node(i))
    for i in range(0, len(cups)-1): 
        refs[cups[i]].nxt = refs[cups[i+1]]
    refs[cups[-1]].nxt = refs[cups[0]]
    head = refs[cups[0]]
    return head, refs

def step(head, refs): 
    x = head.nxt
    y = x.nxt
    z = y.nxt   
    dest = len(refs)-1 if head.val == 1 else head.val-1
    while dest in [x.val, y.val, z.val]: 
        dest = len(refs)-1 if dest == 1 else dest-1
    head.nxt = z.nxt
    refs[dest].nxt, z.nxt = x, refs[dest].nxt
    return head.nxt

def part1(cups): 
    head, refs = build_list(cups)
    for _ in range(0, 100): 
        head = step(head, refs)
    head = refs[1].nxt
    res = 0
    while head.val != 1: 
        res = 10*res+head.val
        head = head.nxt
    return res 

def part2(cups): 
    head, refs = build_list(cups)
    for _ in range(0, int(1e7)): 
        head = step(head, refs)
    head = refs[1]
    x = head.nxt
    y = x.nxt
    return x.val*y.val
    
cups = [int(x) for x in stdin.read()]
cups2 = cups.copy()
for i in range(10, int(1e6)+1): 
    cups2.append(i)
print(part1(cups))
print(part2(cups2))

