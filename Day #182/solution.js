// Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
// BFS connectivity + edge count. Time O(V+E), Space O(V+E).
function isMinimallyConnected(v, edges) {
  if (edges.length !== v - 1) return false;
  const adj = Array.from({ length: v }, () => []);
  for (const [a, b] of edges) {
    adj[a].push(b);
    adj[b].push(a);
  }
  const seen = new Array(v).fill(false);
  const q = [0];
  seen[0] = true;
  let cnt = 1;
  while (q.length) {
    const u = q.shift();
    for (const w of adj[u])
      if (!seen[w]) {
        seen[w] = true;
        cnt++;
        q.push(w);
      }
  }
  return cnt === v;
}

const tree = [[0, 1], [0, 2], [1, 3], [1, 4]];
const cyclic = [[0, 1], [0, 2], [1, 3], [1, 4], [3, 4]];
console.log(isMinimallyConnected(5, tree));
console.log(isMinimallyConnected(5, cyclic));
