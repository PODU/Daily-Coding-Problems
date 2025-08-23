# Day 160: Weighted tree diameter. One DFS; each node returns its longest
# downward branch, while we combine the top two branches. Time O(V+E).
from collections import defaultdict


def tree_diameter(edges, root):
    adj = defaultdict(list)
    for u, v, w in edges:
        adj[u].append((v, w))
        adj[v].append((u, w))

    best = 0

    def dfs(node, parent):
        nonlocal best
        top1 = top2 = 0
        for nb, w in adj[node]:
            if nb == parent:
                continue
            d = w + dfs(nb, node)
            if d > top1:
                top1, top2 = d, top1
            elif d > top2:
                top2 = d
        best = max(best, top1 + top2)
        return top1

    dfs(root, None)
    return best


if __name__ == "__main__":
    edges = [
        ("a", "b", 3), ("a", "c", 5), ("a", "d", 8),
        ("d", "e", 2), ("d", "f", 4),
        ("e", "g", 1), ("e", "h", 1),
    ]
    print(tree_diameter(edges, "a"))  # 17
