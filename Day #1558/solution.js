// Count connected components (friend groups) in an undirected graph via DFS.
// Time O(V+E), Space O(V).
"use strict";

function main() {
  const adj = new Map([
    [0, [1, 2]],
    [1, [0, 5]],
    [2, [0]],
    [3, [6]],
    [4, []],
    [5, [1]],
    [6, [3]],
  ]);

  const visited = new Set();
  let groups = 0;
  for (const start of adj.keys()) {
    if (visited.has(start)) continue;
    groups++;
    const stack = [start];
    visited.add(start);
    while (stack.length) {
      const u = stack.pop();
      for (const v of adj.get(u)) {
        if (!visited.has(v)) {
          visited.add(v);
          stack.push(v);
        }
      }
    }
  }
  console.log(groups);
}

main();
