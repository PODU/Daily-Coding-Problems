# Day 688: 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
# Time O(n + m), Space O(n + m).
import sys


def two_sat(num_vars, clauses):
    N = 2 * num_vars
    graph = [[] for _ in range(N)]

    def node(var, sign):  # literal: sign True=positive, False=negated
        return 2 * var + (0 if sign else 1)

    def neg(n):
        return n ^ 1

    for (v1, s1), (v2, s2) in clauses:
        x, y = node(v1, s1), node(v2, s2)
        # (x OR y) == (not x -> y) and (not y -> x)
        graph[neg(x)].append(y)
        graph[neg(y)].append(x)

    indices = [-1] * N
    low = [0] * N
    onstack = [False] * N
    comp = [-1] * N
    stack = []
    idx = [0]
    cc = [0]

    for start in range(N):
        if indices[start] != -1:
            continue
        work = [(start, 0)]
        while work:
            v, pi = work[-1]
            if pi == 0:
                indices[v] = low[v] = idx[0]
                idx[0] += 1
                stack.append(v)
                onstack[v] = True
            recurse = False
            i = pi
            while i < len(graph[v]):
                w = graph[v][i]
                if indices[w] == -1:
                    work[-1] = (v, i + 1)
                    work.append((w, 0))
                    recurse = True
                    break
                elif onstack[w]:
                    low[v] = min(low[v], indices[w])
                i += 1
            if recurse:
                continue
            if low[v] == indices[v]:
                while True:
                    w = stack.pop()
                    onstack[w] = False
                    comp[w] = cc[0]
                    if w == v:
                        break
                cc[0] += 1
            work.pop()
            if work:
                pv = work[-1][0]
                low[pv] = min(low[pv], low[v])

    # comp ids are in reverse topological order (sink = 0).
    assign = [False] * num_vars
    for v in range(num_vars):
        if comp[2 * v] == comp[2 * v + 1]:
            return None
        assign[v] = comp[2 * v] < comp[2 * v + 1]
    return assign


def main():
    # Variables a=0, b=1, c=2. sign True=positive literal, False=negated.
    # (not c OR b) AND (b OR c) AND (not b OR c) AND (not c OR not a)
    clauses = [
        ((2, False), (1, True)),
        ((1, True), (2, True)),
        ((1, False), (2, True)),
        ((2, False), (0, False)),
    ]
    assign = two_sat(3, clauses)
    if assign is None:
        print("UNSATISFIABLE")
        return
    names = ["a", "b", "c"]
    print(", ".join("%s = %s" % (names[i], "True" if assign[i] else "False")
                    for i in range(3)))


if __name__ == "__main__":
    main()
