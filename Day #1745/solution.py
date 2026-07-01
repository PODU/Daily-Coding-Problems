# Day 1745: Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.

knows_mat = [
    [0, 1, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 0],
    [0, 1, 1, 0],
]


def knows(a, b):
    return knows_mat[a][b] == 1


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


def main():
    print(find_celebrity(4))


if __name__ == "__main__":
    main()
