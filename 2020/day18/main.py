from sys import stdin

def evaluate(expr): 
    vals, op = list(), list()
    cur = 0
    while cur < len(expr): 
        if expr[cur] == '(':
            parens = 1
            nxt = cur+1
            while nxt < len(expr) and parens: 
                parens += expr[nxt] == '('
                parens -= expr[nxt] == ')'
                nxt += 1
            vals.append(evaluate(expr[cur+1:nxt-1]))
            cur = nxt-1
        elif expr[cur].isdigit(): 
            vals.append(int(expr[cur]))
        else: 
            op.append(expr[cur])
        if expr[cur] not in '+*' and op: 
            x, y = vals.pop(), vals.pop()
            if op.pop() == '+': 
                vals.append(x+y)
            else: 
                vals.append(x*y)
        cur += 1
    return vals.pop() 
            
def part1(exprs): 
    return sum(map(evaluate, exprs))

def evaluate2(expr): 
    vals, op = list(), list()
    cur = 0
    while cur < len(expr): 
        if expr[cur] == '(':
            parens = 1
            nxt = cur+1
            while nxt < len(expr) and parens: 
                parens += expr[nxt] == '('
                parens -= expr[nxt] == ')'
                nxt += 1
            vals.append(evaluate2(expr[cur+1:nxt-1]))
            cur = nxt-1
        elif expr[cur].isdigit(): 
            vals.append(int(expr[cur]))
        else: 
            op.append(expr[cur])
        if expr[cur] not in '+*' and op and op[-1] == '+':   
            x, y = vals.pop(), vals.pop()
            if op.pop() == '+': 
                vals.append(x+y)
        cur += 1
    res = 1
    for x in vals: 
        res *= x
    return res

def part2(exprs): 
    return sum(map(evaluate2, exprs))

exprs = [''.join(filter(lambda c: c != ' ', expr)) for expr in stdin.read().splitlines()]
print(part1(exprs))
print(part2(exprs))
