# Day 585: Build histogram heights per row; largest rectangle in histogram via monotonic stack.
# Time O(N*M), Space O(M).

def largest_in_histogram(h):
    st = []
    best = 0
    n = len(h)
    for i in range(n + 1):
        cur = 0 if i == n else h[i]
        while st and h[st[-1]] >= cur:
            height = h[st.pop()]
            left = st[-1] if st else -1
            best = max(best, height * (i - left - 1))
        st.append(i)
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
        best = max(best, largest_in_histogram(h))
    return best


if __name__ == "__main__":
    matrix = [
        [1, 0, 0, 0],
        [1, 0, 1, 1],
        [1, 0, 1, 1],
        [0, 1, 0, 0],
    ]
    print(maximal_rectangle(matrix))
