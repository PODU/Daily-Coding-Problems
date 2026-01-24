# Day 945: Longest path (diameter) in a weighted tree.
# DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
from collections import defaultdict

adj = defaultdict(list)
best = 0


def add_edge(u, v, w):
    adj[u].append((v, w))
    adj[v].append((u, w))


def dfs(node, parent):
    global best
    max1 = max2 = 0  # two largest downward branch lengths
    for nb, w in adj[node]:
        if nb == parent:
            continue
        d = dfs(nb, node) + w
        if d > max1:
            max1, max2 = d, max1
        elif d > max2:
            max2 = d
    best = max(best, max1 + max2)  # path passing through this node
    return max1


if __name__ == "__main__":
    add_edge("a", "b", 3); add_edge("a", "c", 5); add_edge("a", "d", 8)
    add_edge("d", "e", 2); add_edge("d", "f", 4)
    add_edge("e", "g", 1); add_edge("e", "h", 1)
    dfs("a", None)
    print(best)  # 17 (path c -> a -> d -> f)
