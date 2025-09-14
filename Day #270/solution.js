// Day 270: Broadcast time = max shortest-path distance from node 0 (Dijkstra).
// Min-heap Dijkstra over undirected weighted graph; answer = max finite dist. Time O(E log V), Space O(V+E).

class MinHeap {
    constructor() { this.a = []; }
    push(x) {
        this.a.push(x);
        let i = this.a.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (this.a[p][0] <= this.a[i][0]) break;
            [this.a[p], this.a[i]] = [this.a[i], this.a[p]];
            i = p;
        }
    }
    pop() {
        const top = this.a[0], last = this.a.pop();
        if (this.a.length) {
            this.a[0] = last;
            let i = 0;
            const n = this.a.length;
            while (true) {
                let s = i, l = 2 * i + 1, r = 2 * i + 2;
                if (l < n && this.a[l][0] < this.a[s][0]) s = l;
                if (r < n && this.a[r][0] < this.a[s][0]) s = r;
                if (s === i) break;
                [this.a[s], this.a[i]] = [this.a[i], this.a[s]];
                i = s;
            }
        }
        return top;
    }
    get size() { return this.a.length; }
}

function networkDelay(n, edges) {
    const adj = Array.from({ length: n + 1 }, () => []);
    for (const [a, b, t] of edges) {
        adj[a].push([b, t]);
        adj[b].push([a, t]);
    }
    const dist = new Array(n + 1).fill(Infinity);
    dist[0] = 0;
    const pq = new MinHeap();
    pq.push([0, 0]);
    while (pq.size) {
        const [d, u] = pq.pop();
        if (d > dist[u]) continue;
        for (const [v, w] of adj[u]) {
            if (d + w < dist[v]) { dist[v] = d + w; pq.push([dist[v], v]); }
        }
    }
    return Math.max(...dist);
}

const edges = [
    [0, 1, 5], [0, 2, 3], [0, 5, 4],
    [1, 3, 8], [2, 3, 1], [3, 5, 10], [3, 4, 5],
];
console.log(networkDelay(5, edges));
