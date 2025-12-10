# Day 729: Interleave ranked preference lists into one consistent playlist.
# Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
# first-appearance tie-break). Time: O(V + E), Space: O(V + E).
from collections import deque


def interleave(lists):
    adj, indeg, order = {}, {}, []
    for lst in lists:
        for x in lst:
            if x not in indeg:
                indeg[x] = 0
                adj[x] = []
                order.append(x)
        for a, b in zip(lst, lst[1:]):
            if b not in adj[a]:
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
