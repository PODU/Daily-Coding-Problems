// Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
// no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
function isTree(n, edges) {
  if (n === 0) return true;
  const adj = Array.from({ length: n }, () => []);
  for (const [u, v] of edges) { adj[u].push(v); adj[v].push(u); }
  const visited = new Array(n).fill(false);
  visited[0] = true;
  let seen = 1;
  const stack = [[0, -1]];
  while (stack.length) {
    const [u, parent] = stack.pop();
    for (const w of adj[u]) {
      if (!visited[w]) { visited[w] = true; seen++; stack.push([w, u]); }
      else if (w !== parent) return false; // back-edge -> cycle
    }
  }
  return seen === n;
}

const tree = [[0, 1], [1, 2], [1, 3]];
const cyclic = [[0, 1], [1, 2], [2, 0], [2, 3]];
console.log(isTree(4, tree) ? "True" : "False");   // True
console.log(isTree(4, cyclic) ? "True" : "False"); // False
