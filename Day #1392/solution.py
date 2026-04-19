# Day 1392: Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
from collections import deque


def min_moves(target, deadends):
    dead = set(deadends)
    if "000" in dead or target in dead:
        return None
    if target == "000":
        return 0
    visited = {"000"}
    q = deque([("000", 0)])
    while q:
        cur, d = q.popleft()
        for i in range(3):
            for delta in (1, 9):
                nx = cur[:i] + str((int(cur[i]) + delta) % 10) + cur[i + 1:]
                if nx in visited or nx in dead:
                    continue
                if nx == target:
                    return d + 1
                visited.add(nx)
                q.append((nx, d + 1))
    return None


def main():
    deadends = ["100", "020", "001"]
    print(min_moves("333", deadends))


if __name__ == "__main__":
    main()
