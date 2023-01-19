from sys import stdin

def part1(adapters): 
    cnt = [0] * 4
    for i in range(0, len(adapters)-1): 
        cnt[adapters[i+1]-adapters[i]] += 1
    return cnt[1]*cnt[3]

def part2(adapters): 
    dp = [0] * len(adapters)
    dp[-1] = 1
    i = len(adapters)-2
    while i >= 0: 
        j = i+1
        while j < len(adapters) and adapters[j]-adapters[i] <= 3: 
            dp[i] += dp[j]
            j += 1
        i -= 1
    return dp[0]

adapters = [int(x) for x in stdin.read().splitlines()]
adapters.append(0)
adapters.append(max(adapters)+3)
adapters.sort()
print(part1(adapters))
print(part2(adapters))
