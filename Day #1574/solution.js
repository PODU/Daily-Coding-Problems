// Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.

function shortestUniquePrefixes(words) {
  const root = { ch: new Map(), cnt: 0 };
  for (const w of words) {
    let cur = root;
    for (const c of w) {
      if (!cur.ch.has(c)) cur.ch.set(c, { ch: new Map(), cnt: 0 });
      cur = cur.ch.get(c);
      cur.cnt++;
    }
  }
  const res = [];
  for (const w of words) {
    let cur = root;
    let pre = "";
    for (const c of w) {
      cur = cur.ch.get(c);
      pre += c;
      if (cur.cnt === 1) break;
    }
    res.push(pre);
  }
  return res;
}

const words = ["dog", "cat", "apple", "apricot", "fish"];
for (const p of shortestUniquePrefixes(words)) console.log(p);
