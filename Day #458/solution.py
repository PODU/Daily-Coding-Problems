# Day 458: Validate directional (NE/SW/...) rules for consistency.
# Decompose into x/y strict orders; a cycle in either graph = contradiction.
# Time O(R + V) via DFS cycle detection.
from collections import defaultdict


def has_cycle(adj, nodes):
    color = {n: 0 for n in nodes}

    def dfs(u):
        color[u] = 1
        for v in adj[u]:
            if color[v] == 1:
                return True
            if color[v] == 0 and dfs(v):
                return True
        color[u] = 2
        return False

    return any(color[n] == 0 and dfs(n) for n in nodes)


def validate(rules):
    gx, gy = defaultdict(list), defaultdict(list)
    nx, ny = set(), set()

    def less(g, ns, small, big):
        g[small].append(big)
        ns.add(small)
        ns.add(big)

    for r in rules:
        a, d, b = r.split()
        for c in d:
            if c == 'N':
                less(gy, ny, b, a)   # a north of b => a.y > b.y
            elif c == 'S':
                less(gy, ny, a, b)
            elif c == 'E':
                less(gx, nx, b, a)   # a east of b => a.x > b.x
            elif c == 'W':
                less(gx, nx, a, b)
    return not has_cycle(gx, nx) and not has_cycle(gy, ny)


if __name__ == "__main__":
    print("Valid" if validate(["A N B", "B NE C", "C N A"]) else "Invalid")  # Invalid
    print("Valid" if validate(["A NW B", "A N B"]) else "Invalid")          # Valid
