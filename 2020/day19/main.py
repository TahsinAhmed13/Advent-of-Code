from sys import stdin 

def parse_rule(rule): 
    tokens = rule.split(':')
    var = int(tokens[0])
    terminals = list()
    for subrule in tokens[1].split('|'): 
        terminals.append(list())
        for vals in subrule.split(): 
            if vals.isdigit(): 
                terminals[-1].append(int(vals))
            elif vals: 
                terminals[-1].append(vals[1])
    return var, terminals 

def run_pda(rules, w, st): 
    i = 0
    while i < len(w) and st: 
        top = st.pop()
        if isinstance(top, int): 
            for terminal in rules[top]: 
                st2 = st.copy()
                st2.extend(reversed(terminal))
                if run_pda(rules, w[i:], st2): 
                    return True
            return False
        elif top == w[i]: 
            i += 1
        else: 
            return False
    return i >= len(w) and not st

def part1(rules, strs): 
    return len(list(filter(lambda w: run_pda(rules, w, [0]), strs)))

def part2(rules, strs):  
    rules2 = rules.copy()
    extra = ['8: 42 | 42 8', '11: 42 31 | 42 11 31']
    for line in extra: 
        var, terminals = parse_rule(line)
        rules2[var] = terminals
    return part1(rules2, strs)
   
lines = stdin.read().splitlines()
rules = dict()
for line in lines[:lines.index('')]: 
    var, terminals = parse_rule(line)
    rules[var] = terminals
strs = lines[lines.index('')+1:]
print(part1(rules, strs))
print(part2(rules, strs))
