# Day 234: Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
# Time: O(E log E), Space: O(V).


def max_spanning_tree(n, edges):
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def unite(a, b):
        ra, rb = find(a), find(b)
        if ra == rb:
            return False
        parent[ra] = rb
        return True

    total = 0
    chosen = []
    for u, v, w in sorted(edges, key=lambda e: -e[2]):
        if unite(u, v):
            total += w
            chosen.append((u, v))
    return total, chosen


if __name__ == "__main__":
    n = 4
    edges = [(0, 1, 1), (1, 2, 2), (2, 3, 3), (0, 3, 4), (0, 2, 5)]
    total, chosen = max_spanning_tree(n, edges)
    print("Max spanning tree weight:", total)  # 11
    print("Edges:", " ".join(f"({u}-{v})" for u, v in chosen))
