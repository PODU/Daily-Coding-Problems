// PageRank via power iteration. Dangling nodes distribute rank evenly.
// Time O(iters * (N+E)), Space O(N+E).
function pagerank(out, d = 0.85, maxIter = 1000, tol = 1e-9) {
    const N = out.length;
    let rank = new Array(N).fill(1.0 / N);
    for (let iter = 0; iter < maxIter; iter++) {
        const next = new Array(N).fill((1.0 - d) / N);
        let dangling = 0;
        for (let i = 0; i < N; i++) if (out[i].length === 0) dangling += rank[i];
        for (let i = 0; i < N; i++) {
            if (out[i].length === 0) continue;
            const share = rank[i] / out[i].length;
            for (const j of out[i]) next[j] += d * share;
        }
        for (let j = 0; j < N; j++) next[j] += d * dangling / N;
        let diff = 0;
        for (let j = 0; j < N; j++) diff += Math.abs(next[j] - rank[j]);
        rank = next;
        if (diff < tol) break;
    }
    return rank;
}

function main() {
    const out = [[1, 2], [2], [0, 1], [0, 1, 2]];
    const rank = pagerank(out);
    rank.forEach((r, i) => console.log(`${i}: ${r.toFixed(4)}`));
}

main();
