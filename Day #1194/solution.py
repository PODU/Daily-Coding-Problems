# Day 1194: 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
# Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
# Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).
import sys
sys.setrecursionlimit(10000)


class TwoSAT:
    def __init__(self, n):
        self.n = n
        self.adj = [[] for _ in range(2 * n)]
        self.clauses = []
        self.comp = [-1] * (2 * n)
        self.low = [0] * (2 * n)
        self.num = [-1] * (2 * n)
        self.on_stk = [False] * (2 * n)
        self.stk = []
        self.idx = 0
        self.scc = 0

    @staticmethod
    def lit(v, neg):
        return 2 * v + (1 if neg else 0)

    def clause(self, v1, n1, v2, n2):
        self.adj[self.lit(v1, not n1)].append(self.lit(v2, n2))
        self.adj[self.lit(v2, not n2)].append(self.lit(v1, n1))
        self.clauses.append((v1, n1, v2, n2))

    def tarjan(self, u):
        self.low[u] = self.num[u] = self.idx
        self.idx += 1
        self.stk.append(u)
        self.on_stk[u] = True
        for w in self.adj[u]:
            if self.num[w] == -1:
                self.tarjan(w)
                self.low[u] = min(self.low[u], self.low[w])
            elif self.on_stk[w]:
                self.low[u] = min(self.low[u], self.num[w])
        if self.low[u] == self.num[u]:
            while True:
                x = self.stk.pop()
                self.on_stk[x] = False
                self.comp[x] = self.scc
                if x == u:
                    break
            self.scc += 1

    def satisfies(self, val):
        for (v1, n1, v2, n2) in self.clauses:
            a = val[v1] != n1
            b = val[v2] != n2
            if not (a or b):
                return False
        return True

    def solve(self):
        for i in range(2 * self.n):
            if self.num[i] == -1:
                self.tarjan(i)
        for v in range(self.n):
            if self.comp[self.lit(v, False)] == self.comp[self.lit(v, True)]:
                return None
        val = [self.comp[self.lit(v, False)] < self.comp[self.lit(v, True)]
               for v in range(self.n)]
        if not self.satisfies(val):
            val = [self.comp[self.lit(v, True)] < self.comp[self.lit(v, False)]
                   for v in range(self.n)]
        return val


def main():
    ts = TwoSAT(3)  # a=0, b=1, c=2
    # (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
    ts.clause(2, True, 1, False)
    ts.clause(1, False, 2, False)
    ts.clause(1, True, 2, False)
    ts.clause(2, True, 0, True)

    val = ts.solve()
    if val is None:
        print("UNSATISFIABLE")
        return
    print("a = {}, b = {}, c = {}".format(
        "True" if val[0] else "False",
        "True" if val[1] else "False",
        "True" if val[2] else "False"))


if __name__ == "__main__":
    main()
