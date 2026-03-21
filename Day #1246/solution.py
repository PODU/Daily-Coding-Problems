# Day 1246: Maximum weight spanning tree.
# Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
# Time: O(E log E), Space: O(V + E).


class DSU:
    def __init__(self, n):
        self.p = list(range(n))
        self.r = [0] * n

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def unite(self, a, b):
        a, b = self.find(a), self.find(b)
        if a == b:
            return False
        if self.r[a] < self.r[b]:
            a, b = b, a
        self.p[b] = a
        if self.r[a] == self.r[b]:
            self.r[a] += 1
        return True


def max_spanning_tree(n, edges):
    # edges: list of (weight, u, v)
    dsu = DSU(n)
    total = 0
    for w, u, v in sorted(edges, reverse=True):
        if dsu.unite(u, v):
            total += w
    return total


if __name__ == "__main__":
    n = 4
    edges = [(1, 0, 1), (2, 1, 2), (3, 2, 3), (4, 0, 3), (5, 0, 2)]
    print(max_spanning_tree(n, edges))  # 11
