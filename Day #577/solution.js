// Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
function wordCircle(words) {
  const adj = {}; // char -> list of [nextChar, word] in input order
  for (const w of words) {
    const f = w[0];
    if (!adj[f]) adj[f] = [];
    adj[f].push([w[w.length - 1], w]);
  }
  const ptr = {};
  const start = words[0][0];
  const path = [];
  const stack = [[start, null]]; // [char, word used to enter]
  while (stack.length) {
    const v = stack[stack.length - 1][0];
    const lst = adj[v] || [];
    const p = ptr[v] || 0;
    if (p < lst.length) {
      ptr[v] = p + 1;
      const [nxt, w] = lst[p];
      stack.push([nxt, w]);
    } else {
      const [, w] = stack.pop();
      if (w !== null) path.push(w);
    }
  }
  path.reverse();
  return path;
}

const words = ['chair', 'height', 'racket', 'touch', 'tunic'];
const path = wordCircle(words);
console.log(path.join(" --> ") + " --> " + path[0]);
