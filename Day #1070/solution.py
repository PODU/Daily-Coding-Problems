# Day 1070: Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.
from collections import defaultdict

def longest_at_most_2_distinct(nums):
    cnt = defaultdict(int)
    left = 0
    best = 0
    for right, x in enumerate(nums):
        cnt[x] += 1
        while len(cnt) > 2:
            cnt[nums[left]] -= 1
            if cnt[nums[left]] == 0:
                del cnt[nums[left]]
            left += 1
        best = max(best, right - left + 1)
    return best

if __name__ == "__main__":
    print(longest_at_most_2_distinct([2,1,2,3,3,1,3,5]))
