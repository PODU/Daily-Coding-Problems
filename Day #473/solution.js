// Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
// Time: O(V + E), Space: O(V + E).
"use strict";

function main() {
    // Original edges: A->B, B->C
    const edges = [["A", "B"], ["B", "C"]];
    const order = ["A", "B", "C"];

    // Reverse adjacency: v -> u for each u -> v
    const rev = new Map();
    for (const [u, v] of edges) {
        if (!rev.has(v)) rev.set(v, []);
        rev.get(v).push(u);
    }

    // Original chain A -> B -> C becomes A <- B <- C
    console.log(order.join(" <- "));
}

main();
