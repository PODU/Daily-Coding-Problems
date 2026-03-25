# Day 1264: Largest rectangle of 1's in a binary matrix.
# Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
from typing import List


def largest_in_histogram(h: List[int]) -> int:
    best, stack = 0, []
    for i in range(len(h) + 1):
        cur = 0 if i == len(h) else h[i]
        while stack and h[stack[-1]] >= cur:
            height = h[stack.pop()]
            left = stack[-1] if stack else -1
            best = max(best, height * (i - left - 1))
        stack.append(i)
    return best


def maximal_rectangle(m: List[List[int]]) -> int:
    if not m:
        return 0
    cols = len(m[0])
    h = [0] * cols
    best = 0
    for row in m:
        for j in range(cols):
            h[j] = h[j] + 1 if row[j] else 0
        best = max(best, largest_in_histogram(h))
    return best


if __name__ == "__main__":
    matrix = [[1, 0, 0, 0], [1, 0, 1, 1], [1, 0, 1, 1], [0, 1, 0, 0]]
    print(maximal_rectangle(matrix))
