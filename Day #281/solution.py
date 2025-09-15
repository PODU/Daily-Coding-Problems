# Day 281: Fewest bricks cut by a vertical line. Count prefix-sum edge positions;
# answer = rows - max edge frequency. Time O(total bricks), Space O(distinct edges).
from collections import defaultdict


def fewest_cuts(wall):
    edge = defaultdict(int)
    best = 0
    for row in wall:
        s = 0
        for i in range(len(row) - 1):  # skip the far edge
            s += row[i]
            edge[s] += 1
            best = max(best, edge[s])
    return len(wall) - best


if __name__ == "__main__":
    wall = [
        [3, 5, 1, 1], [2, 3, 3, 2], [5, 5],
        [4, 4, 2], [1, 3, 3, 3], [1, 1, 6, 1, 1],
    ]
    print(fewest_cuts(wall))  # 2
