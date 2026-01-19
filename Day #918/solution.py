# Day 918: PageRank via power iteration. Dangling nodes distribute rank evenly.
# Time O(iters * (N+E)), Space O(N+E).
def pagerank(out, d=0.85, max_iter=1000, tol=1e-9):
    N = len(out)
    rank = [1.0 / N] * N
    for _ in range(max_iter):
        nxt = [(1.0 - d) / N] * N
        dangling = sum(rank[i] for i in range(N) if not out[i])
        for i in range(N):
            if not out[i]:
                continue
            share = rank[i] / len(out[i])
            for j in out[i]:
                nxt[j] += d * share
        for j in range(N):
            nxt[j] += d * dangling / N
        diff = sum(abs(nxt[j] - rank[j]) for j in range(N))
        rank = nxt
        if diff < tol:
            break
    return rank

def main():
    out = [[1, 2], [2], [0, 1], [0, 1, 2]]
    rank = pagerank(out)
    for i, r in enumerate(rank):
        print(f"{i}: {r:.4f}")

if __name__ == "__main__":
    main()
