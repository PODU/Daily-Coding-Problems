# Day 1448: Fewest bricks cut by a vertical line through a brick wall.
# Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
# Time O(total bricks), Space O(distinct edges).
from collections import defaultdict
from typing import List


def least_bricks(wall: List[List[int]]) -> int:
    edges = defaultdict(int)
    best = 0
    for row in wall:
        pos = 0
        for brick in row[:-1]:  # skip the far right edge
            pos += brick
            edges[pos] += 1
            best = max(best, edges[pos])
    return len(wall) - best


if __name__ == "__main__":
    wall = [
        [3, 5, 1, 1],
        [2, 3, 3, 2],
        [5, 5],
        [4, 4, 2],
        [1, 3, 3, 3],
        [1, 1, 6, 1, 1],
    ]
    print(least_bricks(wall))  # 2
