// Day 728: Split students into two teams so no enemies share a team.
// Approach: BFS 2-coloring (bipartite check). Returns two teams or false.
// Time: O(V + E), Space: O(V).

function twoTeams(students) {
  const color = new Map();
  const nodes = [...Object.keys(students)].map(Number).sort((a, b) => a - b);
  for (const s of nodes) {
    if (color.has(s)) continue;
    color.set(s, 0);
    const q = [s];
    while (q.length) {
      const u = q.shift();
      for (const v of students[u]) {
        if (!color.has(v)) { color.set(v, color.get(u) ^ 1); q.push(v); }
        else if (color.get(v) === color.get(u)) return false;
      }
    }
  }
  const a = nodes.filter((k) => color.get(k) === 0);
  const b = nodes.filter((k) => color.get(k) === 1);
  return [a, b];
}

function show(students) {
  const res = twoTeams(students);
  if (res === false) { console.log("False"); return; }
  const [a, b] = res;
  console.log(`{${a.join(", ")}} and {${b.join(", ")}}`);
}

show({ 0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3] });
show({ 0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3] });
