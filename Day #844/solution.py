# Day 844: 2-SAT via implication graph + Kosaraju SCC.
# Clause (x OR y) adds edges !x->y and !y->x. SAT iff no var shares an SCC with its negation. O(V+E).
import sys
sys.setrecursionlimit(1 << 20)


class TwoSat:
    def __init__(self, n):
        self.n = n
        self.adj = [[] for _ in range(2 * n)]

    @staticmethod
    def node(var, neg):
        return 2 * var + (1 if neg else 0)

    def add_clause(self, va, na, vb, nb):
        a, b = self.node(va, na), self.node(vb, nb)
        self.adj[a ^ 1].append(b)
        self.adj[b ^ 1].append(a)

    def solve(self):
        n2 = 2 * self.n
        vis = [False] * n2
        order = []

        def dfs1(u):
            stack = [(u, 0)]
            while stack:
                node, i = stack.pop()
                if i == 0:
                    if vis[node]:
                        continue
                    vis[node] = True
                if i < len(self.adj[node]):
                    stack.append((node, i + 1))
                    nxt = self.adj[node][i]
                    if not vis[nxt]:
                        stack.append((nxt, 0))
                else:
                    order.append(node)

        for i in range(n2):
            if not vis[i]:
                dfs1(i)

        radj = [[] for _ in range(n2)]
        for u in range(n2):
            for v in self.adj[u]:
                radj[v].append(u)

        comp = [-1] * n2
        c = 0
        for u in reversed(order):
            if comp[u] == -1:
                stack = [u]
                comp[u] = c
                while stack:
                    x = stack.pop()
                    for v in radj[x]:
                        if comp[v] == -1:
                            comp[v] = c
                            stack.append(v)
                c += 1

        assign = [False] * self.n
        for v in range(self.n):
            if comp[2 * v] == comp[2 * v + 1]:
                return None
            assign[v] = comp[2 * v] > comp[2 * v + 1]
        return assign


if __name__ == "__main__":
    # (¬c OR b) AND (b OR c) AND (¬b OR c) AND (¬c OR ¬a); a=0,b=1,c=2
    ts = TwoSat(3)
    ts.add_clause(2, True, 1, False)
    ts.add_clause(1, False, 2, False)
    ts.add_clause(1, True, 2, False)
    ts.add_clause(2, True, 0, True)
    res = ts.solve()
    if res is None:
        print("False")
    else:
        print("a = {}, b = {}, c = {}".format(*[str(b) for b in res]))
