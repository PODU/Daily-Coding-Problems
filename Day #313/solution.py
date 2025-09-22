# Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
# BFS over <=1000 states. O(1000) time. Returns None when unreachable.
from collections import deque


def open_lock(target, deadends):
    dead = set(deadends)
    start = "000"
    if start in dead:
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
                nx = cur[:i] + str(digit) + cur[i + 1:]
                if nx in dead or nx in dist:
                    continue
                dist[nx] = dist[cur] + 1
                if nx == target:
                    return dist[nx]
                q.append(nx)
    return None


if __name__ == "__main__":
    print(open_lock("123", []))  # 6
