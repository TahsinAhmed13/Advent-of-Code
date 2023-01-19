from sys import stdin

def group_count(group): 
    questions = set()
    for answers in group: 
        questions.update(answers)
    return len(questions)

def part1(groups): 
    return sum(map(group_count, groups))

def group_count2(group): 
    freq = dict()
    for answers in group: 
        for ans in answers: 
            freq[ans] = freq[ans]+1 if ans in freq else 1
    res = 0
    for v in freq.values(): 
        res += v == len(group)
    return res

def part2(group): 
    return sum(map(group_count2, groups))

groups = [[]]
for line in stdin.read().splitlines(): 
    if line: 
        groups[-1].append(line)
    else: 
        groups.append([])
print(part1(groups))
print(part2(groups))
