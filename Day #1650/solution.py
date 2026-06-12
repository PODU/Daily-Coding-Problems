# Day 1650: Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [0] * n

    def find(self, x):
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def unite(self, a, b):
        a, b = self.find(a), self.find(b)
        if a == b:
            return False
        if self.rank[a] < self.rank[b]:
            a, b = b, a
        self.parent[b] = a
        if self.rank[a] == self.rank[b]:
            self.rank[a] += 1
        return True


def max_spanning_tree(n, edges):
    dsu = DSU(n)
    total = 0
    for u, v, w in sorted(edges, key=lambda e: e[2], reverse=True):
        if dsu.unite(u, v):
            total += w
    return total


if __name__ == "__main__":
    edges = [(0, 1, 1), (0, 2, 2), (0, 3, 3), (1, 2, 4), (2, 3, 5)]
    print("Maximum spanning tree weight:", max_spanning_tree(4, edges))
