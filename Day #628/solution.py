# Day 628: Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
# Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
from collections import deque


def open_lock(deadends, target, start="000"):
    dead = set(deadends)
    if start in dead:
        return None
    if start == target:
        return 0
    visited = {start}
    q = deque([start])
    steps = 0
    while q:
        steps += 1
        for _ in range(len(q)):
            cur = q.popleft()
            for w in range(3):
                for d in (1, -1):
                    nxt = cur[:w] + str((int(cur[w]) + d) % 10) + cur[w + 1:]
                    if nxt in dead or nxt in visited:
                        continue
                    if nxt == target:
                        return steps
                    visited.add(nxt)
                    q.append(nxt)
    return None


if __name__ == "__main__":
    r1 = open_lock(["010", "021"], "020")
    print("Min moves to open lock (target 020):", r1)

    r2 = open_lock(["001", "010", "100", "009", "090", "900"], "111")
    print("Impossible case (target 111):", "None" if r2 is None else r2)
