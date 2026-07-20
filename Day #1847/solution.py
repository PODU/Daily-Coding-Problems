# Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
# Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.
import sys

sys.setrecursionlimit(10000)


class TwoSat:
    def __init__(self, n):
        self.n = n
        self.adj = [[] for _ in range(2 * n)]
        self.adjT = [[] for _ in range(2 * n)]

    def add_or(self, u, v):
        self.adj[u ^ 1].append(v); self.adjT[v].append(u ^ 1)
        self.adj[v ^ 1].append(u); self.adjT[u].append(v ^ 1)

    def solve(self):
        N = 2 * self.n
        used = [False] * N
        order = []

        def dfs1(v):
            used[v] = True
            for to in self.adj[v]:
                if not used[to]:
                    dfs1(to)
            order.append(v)

        for i in range(N):
            if not used[i]:
                dfs1(i)

        comp = [-1] * N

        def dfs2(v, c):
            comp[v] = c
            for to in self.adjT[v]:
                if comp[to] == -1:
                    dfs2(to, c)

        c = 0
        for v in reversed(order):
            if comp[v] == -1:
                dfs2(v, c)
                c += 1

        res = [False] * self.n
        for i in range(self.n):
            if comp[2 * i] == comp[2 * i + 1]:
                return None
            res[i] = comp[2 * i] > comp[2 * i + 1]
        return res


def lit(v, neg):
    return 2 * v + neg


def main():
    ts = TwoSat(3)  # a=0, b=1, c=2
    ts.add_or(lit(2, 1), lit(1, 0))  # (¬c OR b)
    ts.add_or(lit(1, 0), lit(2, 0))  # (b OR c)
    ts.add_or(lit(1, 1), lit(2, 0))  # (¬b OR c)
    ts.add_or(lit(2, 1), lit(0, 1))  # (¬c OR ¬a)

    res = ts.solve()
    if res is None:
        print("False")
        return
    names = "abc"
    print(", ".join(f"{names[i]} = {res[i]}" for i in range(3)))


if __name__ == "__main__":
    main()
