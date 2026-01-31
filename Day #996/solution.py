# Day 996: Maximum weight spanning tree.
# Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
# O(E log E) time, O(V) space.

class DSU:
    def __init__(self, n):
        self.p = list(range(n))

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False
        self.p[ra] = rb
        return True


def max_spanning_tree(n, edges):
    dsu = DSU(n)
    total, chosen = 0, []
    for w, u, v in sorted(edges, reverse=True):  # heaviest first
        if dsu.union(u, v):
            total += w
            chosen.append((u, v, w))
    return total, chosen


if __name__ == "__main__":
    edges = [(1, 0, 1), (3, 0, 2), (2, 1, 2), (4, 1, 3), (5, 2, 3)]  # (w,u,v)
    total, _ = max_spanning_tree(4, edges)
    print("Maximum spanning tree weight:", total)  # 12
