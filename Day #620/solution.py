# Day 620: Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
# Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
from collections import defaultdict


def least_bricks(wall):
    edges = defaultdict(int)
    max_count = 0
    for row in wall:
        s = 0
        for i in range(len(row) - 1):
            s += row[i]
            edges[s] += 1
            max_count = max(max_count, edges[s])
    return len(wall) - max_count


if __name__ == "__main__":
    wall = [[3, 5, 1, 1], [2, 3, 3, 2], [5, 5], [4, 4, 2], [1, 3, 3, 3], [1, 1, 6, 1, 1]]
    print(least_bricks(wall))
