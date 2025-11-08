# Day 566: Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
def is_minimally_connected(V, edges):
    if len(edges) != V - 1:
        return False
    parent = list(range(V))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return False  # cycle
        parent[ra] = rb

    root = find(0)
    return all(find(i) == root for i in range(V))


if __name__ == "__main__":
    print("True" if is_minimally_connected(4, [(0, 1), (1, 2), (2, 3)]) else "False")
    print("True" if is_minimally_connected(4, [(0, 1), (1, 2), (2, 3), (3, 0)]) else "False")
