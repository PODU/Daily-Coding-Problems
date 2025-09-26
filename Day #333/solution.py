# Day 333: Celebrity problem: 2-pass. Pass 1 eliminate to one candidate; pass 2 verify.
# Time O(n), Space O(1).
M = [
    [0, 1, 1, 0],  # person0 knows {1,2}
    [0, 0, 1, 0],  # person1 knows {2}
    [0, 0, 0, 0],  # person2 knows {}
    [1, 1, 1, 0],  # person3 knows {0,1,2}
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
    print(find_celebrity(4))
