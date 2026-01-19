# Day 919: Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
# Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
from collections import defaultdict

def least_bricks(wall):
    freq = defaultdict(int)
    best = 0
    for row in wall:
        s = 0
        for i in range(len(row) - 1):
            s += row[i]
            freq[s] += 1
            best = max(best, freq[s])
    return len(wall) - best

def main():
    wall = [[3,5,1,1],[2,3,3,2],[5,5],[4,4,2],[1,3,3,3],[1,1,6,1,1]]
    print(least_bricks(wall))

if __name__ == "__main__":
    main()
