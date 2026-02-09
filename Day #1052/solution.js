// Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
// Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

function divideTeams(students) {
  const color = new Map();
  for (const start of Object.keys(students).map(Number)) {
    if (color.has(start)) continue;
    color.set(start, 0);
    const q = [start];
    while (q.length) {
      const u = q.shift();
      for (const v of students[u]) {
        if (!color.has(v)) { color.set(v, color.get(u) ^ 1); q.push(v); }
        else if (color.get(v) === color.get(u)) return false;
      }
    }
  }
  const a = [], b = [];
  for (const n of Object.keys(students).map(Number)) (color.get(n) === 0 ? a : b).push(n);
  a.sort((x, y) => x - y); b.sort((x, y) => x - y);
  return [a, b];
}

function fmt(res) {
  if (res === false) return "False";
  const setStr = (v) => "{" + v.join(", ") + "}";
  return setStr(res[0]) + " and " + setStr(res[1]);
}

const s1 = { 0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3] };
const s2 = { 0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3] };
console.log(fmt(divideTeams(s1)));
console.log(fmt(divideTeams(s2)));
