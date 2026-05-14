// Minimally-connected = tree: edges == n-1 AND connected. Use BFS.
// Time O(n + e), Space O(n).

function isTree(n, edges) {
  if (edges.length !== n - 1) return false;
  const adj = Array.from({ length: n }, () => []);
  for (const [a, b] of edges) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const visited = new Array(n).fill(false);
  const q = [0];
  visited[0] = true;
  let count = 1;
  let head = 0;
  while (head < q.length) {
    const u = q[head++];
    for (const v of adj[u]) {
      if (!visited[v]) {
        visited[v] = true;
        count++;
        q.push(v);
      }
    }
  }
  return count === n;
}

const n = 4;
const edges = [[0, 1], [1, 2], [1, 3]];
console.log(isTree(n, edges) ? "True" : "False");
