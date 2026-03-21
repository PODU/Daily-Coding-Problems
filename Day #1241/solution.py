# Day 1241: Celebrity problem. Find a single candidate in O(N) using elimination,
# then verify with O(N) knows() calls. Time O(N), Space O(1).


def find_celebrity(n, knows):
    cand = 0
    for i in range(1, n):
        if knows(cand, i):
            cand = i
    for i in range(n):
        if i != cand:
            if knows(cand, i) or not knows(i, cand):
                return -1
    return cand


if __name__ == "__main__":
    # 4 people; person 2 is the celebrity (known by all, knows nobody).
    M = [
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
        [0, 1, 1, 0],
    ]
    knows = lambda a, b: M[a][b] == 1
    print(find_celebrity(4, knows))
