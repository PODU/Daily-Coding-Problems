# Day 1665: Brick wall: prefix-sum edge positions per row, max edge frequency => fewest cuts = rows - maxEdges. O(total bricks) time/space.
from collections import defaultdict

def main():
    wall = [[3,5,1,1],[2,3,3,2],[5,5],[4,4,2],[1,3,3,3],[1,1,6,1,1]]
    freq = defaultdict(int)
    best = 0
    for row in wall:
        s = 0
        for i in range(len(row) - 1):
            s += row[i]
            freq[s] += 1
            best = max(best, freq[s])
    print(len(wall) - best)

if __name__ == "__main__":
    main()
