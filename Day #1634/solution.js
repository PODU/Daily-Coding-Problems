// Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
'use strict';

function differsByOne(a, b) {
  if (a.length !== b.length) return false;
  let diff = 0;
  for (let i = 0; i < a.length; i++)
    if (a[i] !== b[i] && ++diff > 1) return false;
  return diff === 1;
}

function ladder(start, end, dict) {
  const visited = new Set([start]);
  const q = [start];
  const parent = new Map();
  let head = 0;
  while (head < q.length) {
    const cur = q[head++];
    if (cur === end) {
      const path = [];
      let at = cur;
      while (true) {
        path.push(at);
        if (at === start) break;
        at = parent.get(at);
      }
      path.reverse();
      return path;
    }
    for (const w of dict) {
      if (!visited.has(w) && differsByOne(cur, w)) {
        visited.add(w);
        parent.set(w, cur);
        q.push(w);
      }
    }
  }
  return null;
}

function fmt(path) {
  if (path === null) return 'null';
  return '[' + path.map((w) => '"' + w + '"').join(', ') + ']';
}

function main() {
  console.log(fmt(ladder('dog', 'cat', ['dot', 'dop', 'dat', 'cat'])));
  console.log(fmt(ladder('dog', 'cat', ['tod', 'dat', 'dar', 'dot'])));
}

main();
