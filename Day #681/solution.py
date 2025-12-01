# Day 681: BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
from collections import deque


def min_turns():
    snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78}
    ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
    jump = {**snakes, **ladders}

    dist = {1: 0}                       # start placed on 1; do NOT apply 1->38 here
    q = deque([1])
    while q:
        s = q.popleft()
        if s == 100:
            return dist[s]
        for r in range(1, 7):
            nxt = s + r
            if nxt > 100:
                continue
            nxt = jump.get(nxt, nxt)
            if nxt not in dist:
                dist[nxt] = dist[s] + 1
                q.append(nxt)
    return dist.get(100)


if __name__ == "__main__":
    print("Minimum turns:", min_turns())
