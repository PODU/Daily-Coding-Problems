# Day 1760: Sliding window maximum.
# Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.
from collections import deque


def max_sliding_window(a, k):
    res = []
    dq = deque()  # indices, values decreasing
    for i, x in enumerate(a):
        if dq and dq[0] <= i - k:
            dq.popleft()
        while dq and a[dq[-1]] <= x:
            dq.pop()
        dq.append(i)
        if i >= k - 1:
            res.append(a[dq[0]])
    return res


def main():
    a = [10, 5, 2, 7, 8, 7]
    k = 3
    res = max_sliding_window(a, k)
    print("[" + ", ".join(map(str, res)) + "]")


if __name__ == "__main__":
    main()
