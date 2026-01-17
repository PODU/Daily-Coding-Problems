# Day 910: Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
# Bump jumps when current window ends. Time O(n), Space O(1).

def min_jumps(nums):
    n = len(nums)
    if n <= 1:
        return 0
    jumps = cur_end = farthest = 0
    for i in range(n - 1):
        farthest = max(farthest, i + nums[i])
        if i == cur_end:
            jumps += 1
            cur_end = farthest
    return jumps


if __name__ == "__main__":
    nums = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9]
    print(min_jumps(nums))
