# Day 721: Maximum-weight spanning tree.
# Approach: Kruskal with edges sorted by DECREASING weight + union-find.
# Time: O(E log E), Space: O(V + E).

def max_spanning_tree(n, edges):
    parent = list(range(n))
    rnk = [0] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def unite(a, b):
        a, b = find(a), find(b)
        if a == b:
            return False
        if rnk[a] < rnk[b]:
            a, b = b, a
        parent[b] = a
        if rnk[a] == rnk[b]:
            rnk[a] += 1
        return True

    total, used = 0, 0
    for u, v, w in sorted(edges, key=lambda e: -e[2]):
        if unite(u, v):
            total += w
            used += 1
    return total if used == n - 1 else -1


if __name__ == "__main__":
    n = 4
    edges = [(0, 1, 1), (0, 2, 2), (0, 3, 3), (1, 2, 4), (2, 3, 5)]
    print("Maximum spanning tree weight:", max_spanning_tree(n, edges))
