// Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
// union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
function countRegions(grid) {
  const n = grid.length;
  const M = Math.max(...grid.map(r => r.length));
  const g = grid.map(r => r.padEnd(M, ' '));
  const parent = Array.from({ length: n * M * 4 }, (_, i) => i);
  const find = x => { while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; } return x; };
  const union = (a, b) => { parent[find(a)] = find(b); };
  const idx = (i, j, t) => (i * M + j) * 4 + t; // 0=top,1=right,2=bottom,3=left
  for (let i = 0; i < n; i++) for (let j = 0; j < M; j++) {
    const c = g[i][j];
    if (c === '/') { union(idx(i,j,0), idx(i,j,3)); union(idx(i,j,1), idx(i,j,2)); }
    else if (c === '\\') { union(idx(i,j,0), idx(i,j,1)); union(idx(i,j,2), idx(i,j,3)); }
    else { union(idx(i,j,0), idx(i,j,1)); union(idx(i,j,1), idx(i,j,2)); union(idx(i,j,2), idx(i,j,3)); }
    if (j + 1 < M) union(idx(i,j,1), idx(i,j+1,3));
    if (i + 1 < n) union(idx(i,j,2), idx(i+1,j,0));
  }
  let cnt = 0;
  for (let x = 0; x < n * M * 4; x++) if (find(x) === x) cnt++;
  return cnt;
}
const grid = ["\\    /", " \\  /", "  \\/"];
console.log(countRegions(grid)); // 3
