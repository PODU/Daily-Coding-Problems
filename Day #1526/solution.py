# Day 1526: PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
# Time O(iters*(N+E)), Space O(N+E).
def pagerank(nodes, edges, d=0.85, iters=100, tol=1e-9):
    n = len(nodes)
    score = {nd: 1.0 / n for nd in nodes}
    for _ in range(iters):
        dangling = sum(score[nd] for nd in nodes if not edges.get(nd))
        nxt = {nd: (1.0 - d) / n + d * dangling / n for nd in nodes}
        for nd in nodes:
            outs = edges.get(nd, [])
            if not outs:
                continue
            share = d * score[nd] / len(outs)
            for t in outs:
                nxt[t] += share
        diff = sum(abs(nxt[nd] - score[nd]) for nd in nodes)
        score = nxt
        if diff < tol:
            break
    return score


if __name__ == "__main__":
    nodes = ["A", "B", "C", "D"]
    edges = {"A": ["B", "C"], "B": ["C"], "C": ["A"], "D": []}
    edges["D"] = ["C"]
    score = pagerank(nodes, edges)
    for nd in sorted(nodes):
        print(f"{nd} {score[nd]:.6f}")
