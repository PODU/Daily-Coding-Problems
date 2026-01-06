# Day 865: Minimum cost to connect all houses to the plant = MST cost.
# Approach: Kruskal's algorithm with union-find over all pipe edges.
# Time: O(E log E), Space: O(V + E).


def min_cost(pipes):
    nodes = set(pipes)
    edges = []
    for a, nbrs in pipes.items():
        for b, c in nbrs.items():
            nodes.add(b)
            edges.append((c, a, b))
    idx = {n: i for i, n in enumerate(nodes)}
    parent = list(range(len(idx)))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    total = 0
    for c, a, b in sorted(edges):
        ra, rb = find(idx[a]), find(idx[b])
        if ra != rb:
            parent[ra] = rb
            total += c
    return total


if __name__ == "__main__":
    pipes = {
        'plant': {'A': 1, 'B': 5, 'C': 20},
        'A': {'C': 15},
        'B': {'C': 10},
        'C': {}
    }
    print(min_cost(pipes))  # 16
