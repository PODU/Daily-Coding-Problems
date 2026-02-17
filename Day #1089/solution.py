# Day 1089: Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
from collections import deque


def max_sliding_window(a, k):
    dq = deque()  # indices, values decreasing
    res = []
    for i, x in enumerate(a):
        if dq and dq[0] <= i - k:
            dq.popleft()
        while dq and a[dq[-1]] <= x:
            dq.pop()
        dq.append(i)
        if i >= k - 1:
            res.append(a[dq[0]])
    return res


if __name__ == "__main__":
    print(max_sliding_window([10, 5, 2, 7, 8, 7], 3))
