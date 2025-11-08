// Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
function isMinimallyConnected(V, edges) {
    if (edges.length !== V - 1) return false;
    const parent = Array.from({ length: V }, (_, i) => i);

    function find(x) {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    for (const [a, b] of edges) {
        const ra = find(a), rb = find(b);
        if (ra === rb) return false; // cycle
        parent[ra] = rb;
    }
    const root = find(0);
    for (let i = 1; i < V; i++) if (find(i) !== root) return false;
    return true;
}

console.log(isMinimallyConnected(4, [[0, 1], [1, 2], [2, 3]]) ? "True" : "False");
console.log(isMinimallyConnected(4, [[0, 1], [1, 2], [2, 3], [3, 0]]) ? "True" : "False");
