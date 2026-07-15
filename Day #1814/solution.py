# Day 1814: Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
# Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
# Time: O(100*6). Space: O(100).
from collections import deque


def min_turns(snakes, ladders, size=100):
    jump = {}
    jump.update(snakes)
    jump.update(ladders)

    start = 1  # you begin on square 1; jumps trigger only on squares reached by a roll
    dist = {start: 0}
    q = deque([start])
    while q:
        sq = q.popleft()
        if sq == size:
            return dist[sq]
        for d in range(1, 7):
            nxt = sq + d
            if nxt > size:
                continue
            nxt = jump.get(nxt, nxt)
            if nxt not in dist:
                dist[nxt] = dist[sq] + 1
                q.append(nxt)
    return -1


if __name__ == "__main__":
    snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78}
    ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
    print(min_turns(snakes, ladders))
