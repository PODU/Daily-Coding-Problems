# Day 344: Max edges removed so every component has even node count. DFS subtree sizes;
# answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
# Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
# cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
import sys
from collections import defaultdict

sys.setrecursionlimit(10000)


def solve(n, edges, root=1):
    adj = defaultdict(list)
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    answer = 0

    def dfs(u, parent):
        nonlocal answer
        s = 1
        for v in adj[u]:
            if v != parent:
                s += dfs(v, u)
        if u != root and s % 2 == 0:
            answer += 1
        return s

    dfs(root, 0)
    return answer


def main():
    edges = [(1, 2), (1, 3), (3, 4), (3, 5), (4, 6), (4, 7), (4, 8)]
    print(solve(8, edges))


if __name__ == "__main__":
    main()
