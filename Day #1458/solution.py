# Day 1458: Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
# Time: O(100*6). Space: O(100).
from collections import deque


def min_turns():
    snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78}
    ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
    jump = {}
    jump.update(snakes)
    jump.update(ladders)

    def land(s):
        return jump.get(s, s)

    start = land(1)
    dist = {start: 0}
    q = deque([start])
    while q:
        s = q.popleft()
        if s == 100:
            return dist[s]
        for d in range(1, 7):
            nxt = s + d
            if nxt > 100:
                continue
            nxt = land(nxt)
            if nxt not in dist:
                dist[nxt] = dist[s] + 1
                q.append(nxt)
    return -1


def main():
    print(min_turns())


if __name__ == "__main__":
    main()
