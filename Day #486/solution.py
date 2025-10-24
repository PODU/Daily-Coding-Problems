# Day 486: Celebrity problem.
# Two-pointer elimination: one candidate survives in O(n) knows() calls, then
# verify in O(n). Total O(n) time, O(1) space.

# mock relation matrix: KNOWS[a][b] == 1 means a knows b
KNOWS = []


def knows(a, b):
    return KNOWS[a][b] == 1


def find_celebrity(n):
    candidate = 0
    for i in range(1, n):
        if knows(candidate, i):
            candidate = i
    for i in range(n):
        if i == candidate:
            continue
        if knows(candidate, i) or not knows(i, candidate):
            return -1
    return candidate


if __name__ == "__main__":
    # person 2 is the celebrity: knows nobody, everyone knows them
    KNOWS = [
        [0, 1, 1, 0],
        [1, 0, 1, 1],
        [0, 0, 0, 0],
        [1, 1, 1, 0],
    ]
    print(find_celebrity(4))  # 2
