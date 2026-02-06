# Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
# applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
from collections import deque

snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78}
ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100}
jumps = {**snakes, **ladders}


def min_turns():
    dist = {1: 0}  # start at square 1 (mapping applied only on landing after a roll)
    q = deque([1])
    while q:
        s = q.popleft()
        if s == 100:
            return dist[s]
        for d in range(1, 7):
            nx = s + d
            if nx > 100:
                continue
            nx = jumps.get(nx, nx)
            if nx not in dist:
                dist[nx] = dist[s] + 1
                q.append(nx)
    return -1


if __name__ == "__main__":
    print(min_turns())
