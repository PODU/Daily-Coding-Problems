# Day 1184: Interleave ranked lists into one playlist respecting every ordering.
# Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
# Time O(V + E), Space O(V + E).
from collections import deque


def interleave(lists):
    order, known = [], set()
    adj, indeg, edges = {}, {}, set()

    for l in lists:
        for v in l:
            if v not in known:
                known.add(v)
                order.append(v)
                adj.setdefault(v, [])
                indeg.setdefault(v, 0)
        for u, w in zip(l, l[1:]):
            if u != w and (u, w) not in edges:
                edges.add((u, w))
                adj[u].append(w)
                indeg[w] += 1

    q = deque(v for v in order if indeg[v] == 0)
    res = []
    while q:
        v = q.popleft()
        res.append(v)
        for w in adj[v]:
            indeg[w] -= 1
            if indeg[w] == 0:
                q.append(w)
    return res


if __name__ == "__main__":
    lists = [[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]]
    print(interleave(lists))  # [2, 1, 6, 7, 3, 9, 5]
