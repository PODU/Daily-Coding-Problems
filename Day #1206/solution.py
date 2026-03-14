# Day 1206: Validate directional (N/S/E/W) rules for consistency.
# Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
from collections import defaultdict


def has_cycle(adj):
    color = defaultdict(int)  # 0 unvisited, 1 in-stack, 2 done

    def dfs(u):
        color[u] = 1
        for v in adj[u]:
            if color[v] == 1:
                return True
            if color[v] == 0 and dfs(v):
                return True
        color[u] = 2
        return False

    return any(color[u] == 0 and dfs(u) for u in list(adj))


def validate(rules):
    gy, gx = defaultdict(list), defaultdict(list)

    def add(g, u, v):
        g[u].append(v)
        g[v]  # ensure node exists

    for a, d, b in rules:
        if "N" in d: add(gy, a, b)
        if "S" in d: add(gy, b, a)
        if "E" in d: add(gx, a, b)
        if "W" in d: add(gx, b, a)
    return not has_cycle(gy) and not has_cycle(gx)


if __name__ == "__main__":
    rules = [("A", "N", "B"), ("B", "NE", "C"), ("C", "N", "A")]
    print("validates" if validate(rules) else "does not validate")  # does not validate
