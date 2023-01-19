from sys import stdin 

def in_range(fields, x): 
    if not isinstance(fields, list): 
        fields = [fields]
    for field in fields: 
        for param in field.split():
            if not param[0].isdigit(): 
                continue
            tokens = param.split('-')
            a = int(tokens[0])
            b = int(tokens[1])
            if a <= x and x <= b: 
                return True
    return False

def part1(fields, nearby): 
    nums = [x for t in nearby for x in t]
    return sum(list(filter(lambda x: not in_range(fields, x), nums)))
            
def check_ticket(fields, ticket): 
    return all(map(lambda x: in_range(fields, x), ticket)) 

def part2(fields, ticket, nearby): 
    valid = list(filter(lambda t: check_ticket(fields, t), nearby))
    idxes = list(range(0, len(ticket))) 
    pos = dict()
    while len(fields) > 0: 
        nxt = []
        for f in fields: 
            can = []
            for i in idxes: 
                nums = [t[i] for t in valid]    
                ok = all(map(lambda x: in_range(f, x), nums))
                if ok: 
                    can.append(i)
            if len(can) == 1: 
                pos[f] = can[0] 
                idxes.remove(can[0])
            else:
                nxt.append(f)
        fields = nxt
    res = 1
    for k, v in pos.items(): 
        if k.split()[0] == 'departure': 
            res *= ticket[v]
    return res

lines = stdin.read().splitlines()
fields = lines[:lines.index('')]
ticket = [int(x) for x in lines[lines.index('', lines.index('')+1)-1].split(',')]
nearby = [[int(x) for x in t.split(',')] for t in lines[lines.index('', lines.index('')+1)+2:]]
print(part1(fields, nearby))
print(part2(fields, ticket, nearby))
