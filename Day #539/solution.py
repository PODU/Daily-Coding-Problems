# Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
# For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).

def has_cycle(n, edges):
    parent = list(range(n))
    rank = [0] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def unite(a, b):
        a, b = find(a), find(b)
        if a == b:
            return False                 # already connected -> cycle
        if rank[a] < rank[b]:
            a, b = b, a
        parent[b] = a
        if rank[a] == rank[b]:
            rank[a] += 1
        return True

    for a, b in edges:
        if not unite(a, b):
            return True
    return False


if __name__ == "__main__":
    cyclic = [(0, 1), (1, 2), (2, 3), (3, 0)]
    tree = [(0, 1), (1, 2), (2, 3)]
    print(str(has_cycle(4, cyclic)).lower())
    print(str(has_cycle(4, tree)).lower())
