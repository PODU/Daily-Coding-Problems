// Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
// BFS coloring over each component. Time: O(V+E). Space: O(V).
function divideTeams(g) {
  const color = new Map();
  for (const s of Object.keys(g).map(Number)) {
    if (color.has(s)) continue;
    color.set(s, 0);
    const q = [s];
    while (q.length) {
      const u = q.shift();
      for (const v of g[u]) {
        if (!color.has(v)) { color.set(v, color.get(u) ^ 1); q.push(v); }
        else if (color.get(v) === color.get(u)) return false;
      }
    }
  }
  const a = [], b = [];
  for (const [n, c] of color) (c === 0 ? a : b).push(n);
  a.sort((x, y) => x - y); b.sort((x, y) => x - y);
  return [a, b];
}

const students = { 0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3] };
const res = divideTeams(students);
if (res) {
  const [a, b] = res;
  console.log(`{${a.join(", ")}} and {${b.join(", ")}}`);
} else {
  console.log("False");
}
// {0, 1, 4, 5} and {2, 3}
