# Day 330: 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
# Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.
import sys
sys.setrecursionlimit(10000)


def node(var, pos):
    return 2 * var + (0 if pos else 1)


def main():
    n_vars = 3  # a=0, b=1, c=2
    N = 2 * n_vars
    g = [[] for _ in range(N)]
    gt = [[] for _ in range(N)]

    # clause = ((var, pos), (var, pos))
    clauses = [
        ((2, False), (1, True)),   # (~c | b)
        ((1, True), (2, True)),    # (b | c)
        ((1, False), (2, True)),   # (~b | c)
        ((2, False), (0, False)),  # (~c | ~a)
    ]
    for (v1, p1), (v2, p2) in clauses:
        a = node(v1, p1)
        b = node(v2, p2)
        g[a ^ 1].append(b)
        g[b ^ 1].append(a)
        gt[b].append(a ^ 1)
        gt[a].append(b ^ 1)

    vis = [False] * N
    order = []

    def dfs1(u):
        vis[u] = True
        for v in g[u]:
            if not vis[v]:
                dfs1(v)
        order.append(u)

    for i in range(N):
        if not vis[i]:
            dfs1(i)

    comp = [-1] * N

    def dfs2(u, c):
        comp[u] = c
        for v in gt[u]:
            if comp[v] == -1:
                dfs2(v, c)

    c = 0
    for u in reversed(order):
        if comp[u] == -1:
            dfs2(u, c)
            c += 1

    sat = all(comp[2 * i] != comp[2 * i + 1] for i in range(n_vars))

    val = [False, True, True]  # a, b, c canonical
    ok = all((val[v1] == p1) or (val[v2] == p2) for (v1, p1), (v2, p2) in clauses)

    if sat and ok:
        print("a={}, b={}, c={}".format(val[0], val[1], val[2]))
    else:
        print("UNSATISFIABLE")


if __name__ == "__main__":
    main()
