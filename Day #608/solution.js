// Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
// Time: O(N * L * 26). Space: O(N * L).
function wordLadder(start, end, dictionary) {
  const dict = new Set(dictionary);
  if (!dict.has(end)) return null;
  const parent = new Map([[start, null]]);
  const q = [start];
  let head = 0;
  while (head < q.length) {
    const cur = q[head++];
    if (cur === end) {
      const path = [];
      for (let w = end; w !== null; w = parent.get(w)) path.push(w);
      return path.reverse();
    }
    for (let i = 0; i < cur.length; i++) {
      for (let c = 97; c <= 122; c++) {
        const ch = String.fromCharCode(c);
        if (ch === cur[i]) continue;
        const nxt = cur.slice(0, i) + ch + cur.slice(i + 1);
        if (dict.has(nxt) && !parent.has(nxt)) {
          parent.set(nxt, cur);
          q.push(nxt);
        }
      }
    }
  }
  return null;
}

function fmt(path) {
  if (path === null) return "null";
  return "[" + path.map((w) => `"${w}"`).join(", ") + "]";
}

console.log(fmt(wordLadder("dog", "cat", ["dot", "dop", "dat", "cat"])));
console.log(fmt(wordLadder("dog", "cat", ["dot", "tod", "dat", "dar"])));
