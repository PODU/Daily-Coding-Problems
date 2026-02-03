// Word ladder: BFS over words, change one letter per step, track predecessors.
// Time: O(N * L * 26), Space: O(N). Returns shortest path or null.

function ladder(start, end, dictionary) {
  if (start === end) return [start];
  const words = new Set(dictionary);
  const queue = [start];
  const parent = new Map([[start, null]]);
  const visited = new Set([start]);

  while (queue.length) {
    const cur = queue.shift();
    for (let i = 0; i < cur.length; i++) {
      for (let c = 97; c <= 122; c++) {
        const ch = String.fromCharCode(c);
        if (ch === cur[i]) continue;
        const nxt = cur.slice(0, i) + ch + cur.slice(i + 1);
        if (words.has(nxt) && !visited.has(nxt)) {
          visited.add(nxt);
          parent.set(nxt, cur);
          if (nxt === end) {
            const path = [];
            let w = end;
            while (w !== null) { path.push(w); w = parent.get(w); }
            return path.reverse();
          }
          queue.push(nxt);
        }
      }
    }
  }
  return null; // no path
}

function printPath(p) {
  if (p === null) console.log("null");
  else console.log("[" + p.map((w) => `"${w}"`).join(", ") + "]");
}

printPath(ladder("dog", "cat", ["dot", "dop", "dat", "cat"])); // ["dog", "dot", "dat", "cat"]
printPath(ladder("dog", "cat", ["dot", "tod", "dat", "dar"])); // null
