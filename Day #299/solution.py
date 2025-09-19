# Day 299: Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
# Time: O(E log E), Space: O(V + E).
def mst_cost(pipes):
    parent = {node: node for node in pipes}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def unite(a, b):
        ra, rb = find(a), find(b)
        if ra == rb:
            return False
        parent[ra] = rb
        return True

    seen = set()
    edges = []
    for u, nbrs in pipes.items():
        for v, w in nbrs.items():
            a, b = (u, v) if u < v else (v, u)
            if (a, b, w) not in seen:
                seen.add((a, b, w))
                edges.append((w, a, b))
    edges.sort()

    total = 0
    for w, a, b in edges:
        if unite(a, b):
            total += w
    return total


if __name__ == "__main__":
    pipes = {
        'plant': {'A': 1, 'B': 5, 'C': 20},
        'A': {'C': 15},
        'B': {'C': 10},
        'C': {},
    }
    print(mst_cost(pipes))
