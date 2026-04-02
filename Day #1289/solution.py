# Day 1289: PageRank via power iteration with damping factor d.
# Iterate score vector until convergence; dangling nodes spread mass uniformly.
# Time O(iters * (V + E)), Space O(V + E).
def pagerank(nodes, links, d=0.85, iters=100):
    n = len(nodes)
    idx = {node: i for i, node in enumerate(nodes)}
    out_count = {node: len(links.get(node, [])) for node in nodes}
    score = {node: 1.0 / n for node in nodes}

    for _ in range(iters):
        new = {node: (1 - d) / n for node in nodes}
        dangling_sum = sum(score[node] for node in nodes if out_count[node] == 0)
        for node in nodes:
            # dangling mass redistributed evenly
            new[node] += d * dangling_sum / n
        for src in nodes:
            if out_count[src] == 0:
                continue
            share = d * score[src] / out_count[src]
            for dst in links[src]:
                new[dst] += share
        score = new
    return score


if __name__ == "__main__":
    nodes = ["A", "B", "C", "D"]
    links = {"A": ["B", "C"], "B": ["C"], "C": ["A"], "D": ["C"]}
    score = pagerank(nodes, links)
    for node in nodes:
        print(f"{node}: {score[node]:.4f}")
