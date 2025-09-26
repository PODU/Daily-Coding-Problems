# Day 335: PageRank via iterative power method, d=0.85. Iterate until convergence.
# Time: O(iters * (N + E)). Space: O(N + E).

def main():
    names = ["A", "B", "C", "D"]
    N = len(names)
    idx = {n: i for i, n in enumerate(names)}
    edges = [("A", "B"), ("A", "C"), ("B", "C"), ("C", "A"), ("D", "C")]
    incoming = [[] for _ in range(N)]
    out = [0] * N
    for u, v in edges:
        incoming[idx[v]].append(idx[u])
        out[idx[u]] += 1

    d = 0.85
    score = [1.0 / N] * N
    for _ in range(1000):
        nxt = [(1.0 - d) / N] * N
        for j in range(N):
            for i in incoming[j]:
                nxt[j] += d * score[i] / out[i]
        diff = sum(abs(nxt[k] - score[k]) for k in range(N))
        score = nxt
        if diff < 1e-9:
            break
    for i in range(N):
        print(f"{names[i]}: {score[i]:.4f}")


if __name__ == "__main__":
    main()
