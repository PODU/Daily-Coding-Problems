// Day 1184: Interleave ranked lists into one playlist respecting every ordering.
// Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
// Time O(V + E), Space O(V + E).

function interleave(lists) {
    const order = [], known = new Set();
    const adj = new Map(), indeg = new Map(), edges = new Set();

    for (const l of lists) {
        for (const v of l) {
            if (!known.has(v)) {
                known.add(v);
                order.push(v);
                adj.set(v, []);
                indeg.set(v, 0);
            }
        }
        for (let i = 0; i + 1 < l.length; i++) {
            const u = l[i], w = l[i + 1];
            const key = u + "," + w;
            if (u !== w && !edges.has(key)) {
                edges.add(key);
                adj.get(u).push(w);
                indeg.set(w, indeg.get(w) + 1);
            }
        }
    }

    const q = order.filter(v => indeg.get(v) === 0);
    const res = [];
    let head = 0;
    while (head < q.length) {
        const v = q[head++];
        res.push(v);
        for (const w of adj.get(v)) {
            indeg.set(w, indeg.get(w) - 1);
            if (indeg.get(w) === 0) q.push(w);
        }
    }
    return res;
}

const lists = [[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]];
console.log("[" + interleave(lists).join(", ") + "]"); // [2, 1, 6, 7, 3, 9, 5]
