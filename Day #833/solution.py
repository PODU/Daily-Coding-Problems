# Day 833: Celebrity problem: one candidate via elimination, then verify.
# Two-pointer elimination + verification. Time: O(N) knows calls, Space: O(1).

# Demo knows matrix: N=4, person 2 is the celebrity.
M = [
    [0, 1, 1, 0],  # 0 knows 2
    [0, 0, 1, 0],  # 1 knows 2
    [0, 0, 0, 0],  # 2 (celebrity) knows no one
    [0, 1, 1, 0],  # 3 knows 2
]


def knows(a, b):
    return M[a][b] == 1


def find_celebrity(n):
    cand = 0
    for i in range(1, n):
        if knows(cand, i):
            cand = i
    for i in range(n):
        if i == cand:
            continue
        if knows(cand, i) or not knows(i, cand):
            return -1
    return cand


if __name__ == "__main__":
    print(find_celebrity(len(M)))
