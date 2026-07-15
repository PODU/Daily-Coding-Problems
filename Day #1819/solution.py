# Day 1819: Longest weighted path (diameter) in a tree via two DFS passes.
# DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
from collections import defaultdict

g = defaultdict(list)


def add(a, b, w):
    g[a].append((b, w))
    g[b].append((a, w))


def farthest(src):
    parent = {}
    best = [src, 0]  # node, dist
    stack = [(src, 0, None)]
    visited = set()
    while stack:
        u, d, par = stack.pop()
        if u in visited:
            continue
        visited.add(u)
        if par is not None:
            parent[u] = par
        if d > best[1]:
            best = [u, d]
        for v, w in g[u]:
            if v not in visited:
                stack.append((v, d + w, u))
    return best[0], best[1], parent


if __name__ == "__main__":
    add("a", "b", 3); add("a", "c", 5); add("a", "d", 8)
    add("d", "e", 2); add("d", "f", 4); add("e", "g", 1); add("e", "h", 1)

    u, _, _ = farthest("a")        # one endpoint
    v, length, parent = farthest(u)  # other endpoint + parents rooted at u

    path = []
    cur = v
    while True:
        path.append(cur)
        if cur == u:
            break
        cur = parent[cur]

    print(" -> ".join(path) + f", with a length of {length}")
    # c -> a -> d -> f, with a length of 17
