# Day 1237: Count connected components (friend groups) via Union-Find.
# Time O(V + E a(V)), Space O(V).


def count_groups(adj):
    parent = {k: k for k in adj}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        parent[find(a)] = find(b)

    for u, nbrs in adj.items():
        for v in nbrs:
            union(u, v)
    return len({find(x) for x in adj})


if __name__ == "__main__":
    adj = {0: [1, 2], 1: [0, 5], 2: [0], 3: [6], 4: [], 5: [1], 6: [3]}
    print(count_groups(adj))
