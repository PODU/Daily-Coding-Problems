# Day 409: PageRank via power iteration with damping d=0.85.
# Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
# is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).


def page_rank(graph, d=0.85, iters=100, eps=1e-12):
    n = len(graph)
    rank = {node: 1.0 / n for node in graph}
    for _ in range(iters):
        # Dangling nodes (no out-links) leak rank; redistribute it evenly.
        dangling = sum(rank[node] for node, outs in graph.items() if not outs)
        nxt = {node: (1.0 - d) / n + d * dangling / n for node in graph}
        for node, outs in graph.items():
            if not outs:
                continue
            share = rank[node] / len(outs)
            for nbr in outs:
                nxt[nbr] += d * share
        diff = sum(abs(nxt[node] - rank[node]) for node in graph)
        rank = nxt
        if diff < eps:
            break
    return rank


if __name__ == "__main__":
    graph = {
        "A": ["B", "C"],
        "B": ["C"],
        "C": ["A"],
    }
    rank = page_rank(graph)
    for node in sorted(rank):
        print(f"{node}: {rank[node]:.4f}")
