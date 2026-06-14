# Day 1660: DFS subtree sizes; count non-root subtrees with even size = max edges removable.
# Time O(n), Space O(n).
import sys
sys.setrecursionlimit(10000)

def main():
    n = 8
    g = [[] for _ in range(n + 1)]
    edges = [(1,2),(1,3),(3,4),(3,5),(4,6),(4,7),(4,8)]
    for a, b in edges:
        g[a].append(b); g[b].append(a)
    ans = 0
    def dfs(u, p):
        nonlocal ans
        sz = 1
        for v in g[u]:
            if v != p:
                sz += dfs(v, u)
        if p != -1 and sz % 2 == 0:
            ans += 1
        return sz
    dfs(1, -1)
    print(ans)

main()
