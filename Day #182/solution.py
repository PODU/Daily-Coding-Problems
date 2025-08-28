# Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
# BFS connectivity + edge count. Time O(V+E), Space O(V+E).
from collections import deque
from typing import List, Tuple


def is_minimally_connected(v: int, edges: List[Tuple[int, int]]) -> bool:
    if len(edges) != v - 1:
        return False
    adj = [[] for _ in range(v)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    seen = [False] * v
    q = deque([0])
    seen[0] = True
    cnt = 1
    while q:
        u = q.popleft()
        for w in adj[u]:
            if not seen[w]:
                seen[w] = True
                cnt += 1
                q.append(w)
    return cnt == v


if __name__ == "__main__":
    tree = [(0, 1), (0, 2), (1, 3), (1, 4)]
    cyclic = [(0, 1), (0, 2), (1, 3), (1, 4), (3, 4)]
    print(is_minimally_connected(5, tree))
    print(is_minimally_connected(5, cyclic))
