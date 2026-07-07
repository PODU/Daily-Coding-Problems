// Build origin->sorted destinations; DFS backtracking trying lexicographically
// smallest dest first; first path using all flights is the answer.
// Time O(E!) worst case, Space O(E).
function findItinerary(flights, start) {
    const total = flights.length;
    const graph = new Map();
    for (const [o, d] of flights) {
        if (!graph.has(o)) graph.set(o, []);
        graph.get(o).push(d);
    }
    for (const arr of graph.values()) arr.sort();

    function dfs(node, path) {
        if (path.length === total + 1) return path.slice();
        const dests = graph.get(node) || [];
        for (let i = 0; i < dests.length; i++) {
            if (dests[i] === null) continue;
            const d = dests[i];
            dests[i] = null;
            path.push(d);
            const res = dfs(d, path);
            if (res !== null) return res;
            path.pop();
            dests[i] = d;
        }
        return null;
    }
    return dfs(start, [start]);
}

function show(r) {
    if (r === null) { console.log("null"); return; }
    console.log("[" + r.map(x => `'${x}'`).join(", ") + "]");
}

show(findItinerary([['SFO','HKO'],['YYZ','SFO'],['YUL','YYZ'],['HKO','ORD']], 'YUL'));
show(findItinerary([['SFO','COM'],['COM','YYZ']], 'COM'));
show(findItinerary([['A','B'],['A','C'],['B','C'],['C','A']], 'A'));
