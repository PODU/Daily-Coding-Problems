# Day 1685: Merge ranked preference lists -> topological sort (Kahn's BFS, FIFO,
# first-seen tie-break). Each adjacent pair in a list is an edge. Time O(V+E).
from collections import deque, defaultdict


def interleave(lists):
    order = []
    seen = set()
    adj = defaultdict(list)
    edges = set()
    indeg = defaultdict(int)

    for lst in lists:
        for x in lst:
            if x not in seen:
                seen.add(x)
                order.append(x)
                indeg[x]
        for a, b in zip(lst, lst[1:]):
            if (a, b) not in edges:
                edges.add((a, b))
                adj[a].append(b)
                indeg[b] += 1

    q = deque(x for x in order if indeg[x] == 0)
    res = []
    while q:
        u = q.popleft()
        res.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return res


if __name__ == "__main__":
    print(interleave([[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]]))
    # [2, 1, 6, 7, 3, 9, 5]
