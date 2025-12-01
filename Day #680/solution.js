// Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).
"use strict";

function shortestUniquePrefixes(words) {
  // Build trie: each node is a Map; store pass-count on a symbol key.
  const COUNT = Symbol("count");
  const rootNode = new Map();
  for (const w of words) {
    let node = rootNode;
    for (const c of w) {
      if (!node.has(c)) node.set(c, new Map([[COUNT, 0]]));
      const child = node.get(c);
      child.set(COUNT, child.get(COUNT) + 1);
      node = child;
    }
  }
  const res = [];
  for (const w of words) {
    let node = rootNode;
    let pre = "";
    for (const c of w) {
      node = node.get(c);
      pre += c;
      if (node.get(COUNT) === 1) break;
    }
    res.push(pre);
  }
  return res;
}

console.log(shortestUniquePrefixes(["dog", "cat", "apple", "apricot", "fish"]));
