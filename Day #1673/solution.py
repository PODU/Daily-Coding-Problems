# Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
# BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
from collections import deque


def open_lock(target, deadends):
    dead = set(deadends)
    start = "000"
    if start in dead or target in dead:
        return None
    if start == target:
        return 0
    q = deque([start])
    dist = {start: 0}
    while q:
        cur = q.popleft()
        for i in range(3):
            for d in (-1, 1):
                digit = (int(cur[i]) + d) % 10
                nxt = cur[:i] + str(digit) + cur[i + 1:]
                if nxt in dead or nxt in dist:
                    continue
                dist[nxt] = dist[cur] + 1
                if nxt == target:
                    return dist[nxt]
                q.append(nxt)
    return None


if __name__ == "__main__":
    print(open_lock("345", ["333"]))  # 12
