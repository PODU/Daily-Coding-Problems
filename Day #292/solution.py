# Day 292: Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
# Time O(V+E), Space O(V).
from collections import deque


def two_color(g, n):
    color = [-1] * n
    for s in range(n):
        if color[s] != -1:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in g[u]:
                if color[v] == -1:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return None
    return color


def solve(g, n):
    color = two_color(g, n)
    if color is None:
        print("False")
        return
    a = sorted(i for i in range(n) if color[i] == 0)
    b = sorted(i for i in range(n) if color[i] == 1)
    fmt = lambda s: "{" + ", ".join(map(str, s)) + "}"
    print(f"{fmt(a)} and {fmt(b)}")


if __name__ == "__main__":
    g1 = {0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3]}
    g2 = {0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3]}
    solve(g1, 6)
    solve(g2, 6)
