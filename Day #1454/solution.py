# Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
# no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
from typing import List, Tuple


def is_tree(n: int, edges: List[Tuple[int, int]]) -> bool:
    if n == 0:
        return True
    adj = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    visited = [False] * n
    visited[0] = True
    seen = 1
    stack = [(0, -1)]
    while stack:
        u, parent = stack.pop()
        for w in adj[u]:
            if not visited[w]:
                visited[w] = True
                seen += 1
                stack.append((w, u))
            elif w != parent:
                return False  # back-edge -> cycle
    return seen == n


if __name__ == "__main__":
    tree = [(0, 1), (1, 2), (1, 3)]
    cyclic = [(0, 1), (1, 2), (2, 0), (2, 3)]
    print("True" if is_tree(4, tree) else "False")    # True
    print("True" if is_tree(4, cyclic) else "False")  # False
