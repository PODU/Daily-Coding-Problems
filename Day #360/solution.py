# Day 360: Merge ranked lists via topological sort: edge a->b for consecutive a,b in any list; Kahn's with FIFO queue. O(V+E).
from collections import deque, defaultdict

def merge(lists):
    order = []                 # first-appearance order
    seen = set()
    adj = defaultdict(list)
    indeg = defaultdict(int)
    for l in lists:
        for x in l:
            if x not in seen:
                seen.add(x)
                order.append(x)
                indeg[x] += 0
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
    lists = [[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]]
    print("[" + ", ".join(map(str, merge(lists))) + "]")
