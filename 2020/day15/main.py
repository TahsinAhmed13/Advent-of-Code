from sys import stdin

def play(nums, turn): 
    if turn <= len(nums): 
        return nums[turn-1]
    last = dict()
    for i in range(0, len(nums)-1): 
        last[nums[i]] = i+1
    cur = nums[-1]
    for i in range(len(nums), turn): 
        nxt = i-last[cur] if cur in last else 0
        last[cur] = i
        cur = nxt
    return cur

def part1(nums): 
    return play(nums, 2020)

def part2(nums): 
    return play(nums, int(3e7))

nums = [int(x) for x in stdin.readline().split(',')]
print(part1(nums))
print(part2(nums))
