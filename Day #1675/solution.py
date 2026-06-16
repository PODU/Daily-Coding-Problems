# Day 1675: Detect a cycle in an undirected graph via Union-Find.
# Union endpoints; a cycle exists if an edge joins already-connected nodes.
# Time O(E*alpha(V)), Space O(V).


def has_cycle(n, edges):
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return True
        parent[ra] = rb
    return False


if __name__ == "__main__":
    print(has_cycle(4, [(0, 1), (1, 2), (2, 3), (3, 0)]))  # True
    print(has_cycle(4, [(0, 1), (1, 2), (2, 3)]))          # False
