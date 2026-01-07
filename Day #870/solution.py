# Day 870: Largest rectangle of 1's in a binary matrix.
# Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
# Time: O(N*M), Space: O(M).


def largest_in_hist(h):
    stack = []
    best = 0
    n = len(h)
    for i in range(n + 1):
        cur = 0 if i == n else h[i]
        while stack and h[stack[-1]] >= cur:
            height = h[stack.pop()]
            width = i if not stack else i - stack[-1] - 1
            best = max(best, height * width)
        stack.append(i)
    return best


def maximal_rectangle(mat):
    if not mat:
        return 0
    m = len(mat[0])
    h = [0] * m
    best = 0
    for row in mat:
        for j in range(m):
            h[j] = h[j] + 1 if row[j] else 0
        best = max(best, largest_in_hist(h))
    return best


if __name__ == "__main__":
    mat = [[1, 0, 0, 0],
           [1, 0, 1, 1],
           [1, 0, 1, 1],
           [0, 1, 0, 0]]
    print(maximal_rectangle(mat))  # 4
