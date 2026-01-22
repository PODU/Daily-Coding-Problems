# Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
# 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns None if blocked.
from collections import deque


def min_moves(target, dead):
    dead = set(dead)
    start = "000"
    if start in dead or target in dead:
        return None
    if start == target:
        return 0
    visited = {start}
    q = deque([(start, 0)])
    while q:
        cur, d = q.popleft()
        for i in range(3):
            for delta in (1, -1):
                digit = (int(cur[i]) + delta) % 10
                nx = cur[:i] + str(digit) + cur[i + 1:]
                if nx in dead or nx in visited:
                    continue
                if nx == target:
                    return d + 1
                visited.add(nx)
                q.append((nx, d + 1))
    return None


if __name__ == "__main__":
    print(min_moves("123", []))  # 6
    dead = ["100", "900", "010", "090", "001", "009"]  # seal off 000
    print(min_moves("111", dead))  # None
