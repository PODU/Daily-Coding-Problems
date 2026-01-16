# Day 905: Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
# Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
from collections import defaultdict


def can_chain(words):
    indeg = defaultdict(int)
    outdeg = defaultdict(int)
    adj = defaultdict(list)
    radj = defaultdict(list)
    nodes = set()
    for w in words:
        a, b = w[0], w[-1]
        outdeg[a] += 1
        indeg[b] += 1
        adj[a].append(b)
        radj[b].append(a)
        nodes.add(a)
        nodes.add(b)
    for n in nodes:
        if indeg[n] != outdeg[n]:
            return False
    if not nodes:
        return True
    start = next(iter(nodes))

    def dfs(g, u, seen):
        seen.add(u)
        for v in g[u]:
            if v not in seen:
                dfs(g, v, seen)

    seen = set()
    dfs(adj, start, seen)
    if any(n not in seen for n in nodes):
        return False
    seen = set()
    dfs(radj, start, seen)
    if any(n not in seen for n in nodes):
        return False
    return True


if __name__ == "__main__":
    words = ["chair", "height", "racket", "touch", "tunic"]
    print(can_chain(words))
