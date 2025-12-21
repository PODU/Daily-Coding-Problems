# Day 778: Interleave ranked preference lists -> topological sort (Kahn's).
# Consecutive items in each list become edges. FIFO queue + first-seen order.
# O(V + E).
from collections import deque, defaultdict


def interleave(lists):
    order = []
    seen = set()
    adj = defaultdict(list)
    indeg = defaultdict(int)
    for l in lists:
        for x in l:
            if x not in seen:
                seen.add(x)
                order.append(x)
                indeg.setdefault(x, 0)
        for a, b in zip(l, l[1:]):
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
