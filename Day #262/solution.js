// Day 262: Find all bridges in a connected undirected graph.
// Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
// An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

class BridgeFinder {
  constructor(n) {
    this.n = n;
    this.adj = Array.from({ length: n }, () => []);
    this.timer = 0;
  }

  addEdge(u, v) {
    this.adj[u].push(v);
    this.adj[v].push(u);
  }

  findBridges() {
    const disc = new Array(this.n).fill(0);
    const low = new Array(this.n).fill(0);
    const bridges = [];

    const dfs = (u, parent) => {
      disc[u] = low[u] = ++this.timer;
      let skippedParent = false;
      for (const v of this.adj[u]) {
        if (v === parent && !skippedParent) { skippedParent = true; continue; }
        if (disc[v] === 0) {
          dfs(v, u);
          low[u] = Math.min(low[u], low[v]);
          if (low[v] > disc[u]) bridges.push([Math.min(u, v), Math.max(u, v)]);
        } else {
          low[u] = Math.min(low[u], disc[v]);
        }
      }
    };

    for (let i = 0; i < this.n; i++) if (disc[i] === 0) dfs(i, -1);
    bridges.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
    return bridges;
  }
}

function main() {
  const g = new BridgeFinder(5);
  [[0, 1], [1, 2], [2, 0], [1, 3], [3, 4]].forEach(([u, v]) => g.addEdge(u, v));
  const bridges = g.findBridges();
  console.log("Bridges: [" + bridges.map(([a, b]) => `(${a}, ${b})`).join(", ") + "]");
}

main();
