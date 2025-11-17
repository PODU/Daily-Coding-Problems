// Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
// Approach: Dijkstra from node 0, return the largest distance. Time O(E log V) with a binary heap.
'use strict';

// Minimal binary min-heap keyed on distance.
class MinHeap {
    constructor() { this.a = []; }
    push(x) {
        const a = this.a; a.push(x); let i = a.length - 1;
        while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break; [a[p], a[i]] = [a[i], a[p]]; i = p; }
    }
    pop() {
        const a = this.a, top = a[0], last = a.pop();
        if (a.length) { a[0] = last; let i = 0; const n = a.length;
            for (;;) { let l = 2 * i + 1, r = l + 1, s = i;
                if (l < n && a[l][0] < a[s][0]) s = l;
                if (r < n && a[r][0] < a[s][0]) s = r;
                if (s === i) break; [a[s], a[i]] = [a[i], a[s]]; i = s; } }
        return top;
    }
    get size() { return this.a.length; }
}

function broadcastTime(n, edges) {
    const adj = Array.from({ length: n + 1 }, () => []);
    for (const [a, b, t] of edges) adj[a].push([b, t]);
    const dist = new Array(n + 1).fill(Infinity);
    dist[0] = 0;
    const pq = new MinHeap();
    pq.push([0, 0]);
    while (pq.size) {
        const [d, u] = pq.pop();
        if (d > dist[u]) continue;
        for (const [v, w] of adj[u])
            if (d + w < dist[v]) { dist[v] = d + w; pq.push([dist[v], v]); }
    }
    return Math.max(...dist);
}

function main() {
    const N = 5;
    const edges = [
        [0, 1, 5], [0, 2, 3], [0, 5, 4], [1, 3, 8],
        [2, 3, 1], [3, 5, 10], [3, 4, 5],
    ];
    console.log(broadcastTime(N, edges)); // 9
}

main();
