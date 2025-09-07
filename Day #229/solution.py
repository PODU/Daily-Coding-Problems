# Day 229: Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
# Time: O(100 * 6), Space: O(100).
from collections import deque


def min_turns():
    snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78}
    ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
    jump = {**snakes, **ladders}
    dist = {}
    start = jump.get(1, 1)
    q = deque([start])
    dist[start] = 0
    while q:
        p = q.popleft()
        if p == 100:
            return dist[p]
        for d in range(1, 7):
            np = p + d
            if np > 100:
                continue
            np = jump.get(np, np)
            if np not in dist:
                dist[np] = dist[p] + 1
                q.append(np)
    return dist.get(100)


if __name__ == "__main__":
    print("Minimum turns:", min_turns())
