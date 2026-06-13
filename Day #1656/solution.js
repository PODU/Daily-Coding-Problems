// Trie autocomplete: insert words, DFS from prefix node in child-insertion order
// (Map preserves order). O(total chars) build, O(matches) query; O(total chars) space.
class Node {
  constructor() { this.children = new Map(); this.word = null; }
}

class Trie {
  constructor() { this.root = new Node(); }
  insert(w) {
    let n = this.root;
    for (const c of w) {
      if (!n.children.has(c)) n.children.set(c, new Node());
      n = n.children.get(c);
    }
    n.word = w;
  }
  autocomplete(s) {
    let n = this.root;
    for (const c of s) {
      if (!n.children.has(c)) return [];
      n = n.children.get(c);
    }
    const out = [];
    const dfs = node => {
      if (node.word !== null) out.push(node.word);
      for (const child of node.children.values()) dfs(child);
    };
    dfs(n);
    return out;
  }
}

const t = new Trie();
for (const w of ["dog", "deer", "deal"]) t.insert(w);
console.log("[" + t.autocomplete("de").join(", ") + "]");
