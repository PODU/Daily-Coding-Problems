// Shortest uphill-then-downhill cycle from home (node 0).
// Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
'use strict';

function dijkstra(adj, src, n) {
    const dist = new Array(n).fill(Infinity);
    dist[src] = 0;
    // simple binary-heap-free PQ via array (n is tiny); use array + sort-min for clarity
    const pq = [[0, src]];
    while (pq.length) {
        // extract min
        let mi = 0;
        for (let i = 1; i < pq.length; i++) if (pq[i][0] < pq[mi][0]) mi = i;
        const [d, u] = pq.splice(mi, 1)[0];
        if (d > dist[u]) continue;
        for (const [v, w] of adj[u]) {
            if (d + w < dist[v]) {
                dist[v] = d + w;
                pq.push([dist[v], v]);
            }
        }
    }
    return dist;
}

function shortestRoute(elevations, paths) {
    let n = 0;
    for (const k of Object.keys(elevations)) n = Math.max(n, Number(k) + 1);
    const up = Array.from({ length: n }, () => []);
    const downRev = Array.from({ length: n }, () => []);
    for (const [[u, v], w] of paths) {
        if (elevations[v] > elevations[u]) up[u].push([v, w]);
        else if (elevations[v] < elevations[u]) downRev[v].push([u, w]);
    }
    const upD = dijkstra(up, 0, n);
    const dnD = dijkstra(downRev, 0, n);
    let best = Infinity;
    for (let p = 1; p < n; p++)
        if (upD[p] < Infinity && dnD[p] < Infinity)
            best = Math.min(best, upD[p] + dnD[p]);
    return best;
}

const elevations = { 0: 5, 1: 25, 2: 15, 3: 20, 4: 10 };
const paths = [
    [[0, 1], 10], [[0, 2], 8], [[0, 3], 15], [[1, 3], 12],
    [[2, 4], 10], [[3, 4], 5], [[3, 0], 17], [[4, 0], 10]
];
console.log(shortestRoute(elevations, paths));
