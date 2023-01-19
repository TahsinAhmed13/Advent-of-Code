from sys import stdin

def parse_mask(mask): 
    a, b = 0, 0
    for i in range(0, len(mask)): 
        if mask[-i-1] == '1': 
            a += 1<<i
        if mask[-i-1] != '0': 
            b += 1<<i
    return (a, b)
             
def part1(instrs): 
    mask = (0, 0)
    mem = dict()
    for instr in instrs: 
        tokens = instr.split()
        if tokens[0] == 'mask': 
            mask = parse_mask(tokens[2])
        else: 
            mem[int(tokens[0][4:-1])] = (int(tokens[2]) | mask[0]) & mask[1] 
    return sum(mem.values())

def get_addrs(mask, addr): 
    base = 0
    pos = list()
    for i in range(0, len(mask)): 
        if mask[-i-1] == '0': 
            base += addr & (1<<i)
        elif mask[-i-1] == '1': 
            base += 1<<i
        else: 
            pos.append(i)
    res = list()
    for i in range(0, 1<<len(pos)): 
        val = base 
        for j in range(0, len(pos)): 
            if i&(1<<j): 
                val += 1<<pos[j]
        res.append(val)
    return res

def part2(instrs): 
    mask = ''
    mem = dict()
    for instr in instrs: 
        tokens = instr.split()
        if tokens[0] == 'mask': 
            mask = tokens[2]
        else: 
            addr = int(tokens[0][4:-1])
            val = int(tokens[2])
            for x in get_addrs(mask, addr): 
                mem[x] = val 
    return sum(mem.values())
    
instrs = stdin.read().splitlines()
print(part1(instrs))
print(part2(instrs))
