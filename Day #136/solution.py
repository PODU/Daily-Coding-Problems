# Day 136: Largest rectangle of 1's: per-row histogram + largest-rectangle-in-histogram via monotonic stack.
# Time O(N*M), Space O(M).

def maximal_rectangle(mat):
    if not mat or not mat[0]:
        return 0
    m = len(mat[0])
    h = [0] * m
    best = 0
    for row in mat:
        for j in range(m):
            h[j] = h[j] + 1 if row[j] else 0
        st = []
        for j in range(m + 1):
            cur = 0 if j == m else h[j]
            while st and h[st[-1]] >= cur:
                height = h[st.pop()]
                width = j if not st else j - st[-1] - 1
                best = max(best, height * width)
            st.append(j)
    return best


if __name__ == "__main__":
    mat = [[1, 0, 0, 0], [1, 0, 1, 1], [1, 0, 1, 1], [0, 1, 0, 0]]
    print(maximal_rectangle(mat))
