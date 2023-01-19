from sys import stdin
from collections import deque
from itertools import product

def part1(nums): 
    dq = deque(nums[:25])
    for x in nums[25:]: 
        ok = False
        for a, b in product(dq, dq): 
            if a != b and a+b == x: 
                ok = True
                break 
        if not ok: 
            return x
        dq.popleft()
        dq.append(x)

def part2(nums): 
    target = part1(nums)
    total = 0
    r = 0
    for i in range(0, len(nums)): 
        while r < len(nums) and total < target: 
            total += nums[r]
            r += 1 
        if total == target: 
            return min(nums[i:r])+max(nums[i:r])
        total -= nums[i]

nums = [int(x) for x in stdin.read().splitlines()]
print(part1(nums))
print(part2(nums))
