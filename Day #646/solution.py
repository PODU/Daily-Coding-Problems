# Day 646: Count friend groups = connected components in an undirected graph.
# Approach: Union-Find (DSU) with path compression + union by rank.
# Time: O(V + E * alpha(V)), Space: O(V).

def count_groups(adj):
    parent = {u: u for u in adj}
    rank = {u: 0 for u in adj}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def unite(a, b):
        a, b = find(a), find(b)
        if a == b:
            return
        if rank[a] < rank[b]:
            a, b = b, a
        parent[b] = a
        if rank[a] == rank[b]:
            rank[a] += 1

    for u, nbrs in adj.items():
        for v in nbrs:
            unite(u, v)
    return len({find(u) for u in adj})


if __name__ == "__main__":
    adj = {0: [1, 2], 1: [0, 5], 2: [0], 3: [6], 4: [], 5: [1], 6: [3]}
    print(count_groups(adj))  # 3
