# Day 18: Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.
from collections import deque


def max_sliding_window(nums, k):
    dq = deque()  # indices, decreasing values
    res = []
    for i, v in enumerate(nums):
        if dq and dq[0] <= i - k:
            dq.popleft()
        while dq and nums[dq[-1]] <= v:
            dq.pop()
        dq.append(i)
        if i >= k - 1:
            res.append(nums[dq[0]])
    return res


if __name__ == "__main__":
    nums = [10, 5, 2, 7, 8, 7]
    k = 3
    print(max_sliding_window(nums, k))
