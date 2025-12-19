# Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
# Time: O(n), Space: O(k). Prints each window max as it is computed.
from collections import deque


def sliding_max(a, k):
    dq = deque()   # indices, values decreasing
    out = []
    for i, x in enumerate(a):
        while dq and a[dq[-1]] <= x:
            dq.pop()
        dq.append(i)
        if dq[0] <= i - k:
            dq.popleft()
        if i >= k - 1:
            out.append(a[dq[0]])
    print(out)


if __name__ == "__main__":
    sliding_max([10, 5, 2, 7, 8, 7], 3)   # [10, 7, 8, 8]
