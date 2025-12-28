// Shortest unique prefix via trie of char frequency counts.
// Walk each word until count==1. Time O(total chars), Space O(total chars).

function buildTrie(words) {
  const root = new Map();
  for (const w of words) {
    let cur = root;
    for (const c of w) {
      if (!cur.has(c)) cur.set(c, { cnt: 0, next: new Map() });
      const node = cur.get(c);
      node.cnt++;
      cur = node.next;
    }
  }
  return root;
}

function shortestPrefix(root, w) {
  let cur = root;
  let pre = "";
  for (const c of w) {
    const node = cur.get(c);
    pre += c;
    cur = node.next;
    if (node.cnt === 1) break;
  }
  return pre;
}

function main() {
  const words = ["dog", "cat", "apple", "apricot", "fish"];
  const root = buildTrie(words);
  const res = words.map((w) => shortestPrefix(root, w));
  console.log("[" + res.join(", ") + "]");
}

main();
