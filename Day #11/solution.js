// Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
// Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
class TrieNode {
  constructor() {
    this.ch = new Map();
    this.order = -1;
  }
}

class Trie {
  constructor() {
    this.root = new TrieNode();
    this.counter = 0;
  }
  insert(w) {
    let cur = this.root;
    for (const c of w) {
      if (!cur.ch.has(c)) cur.ch.set(c, new TrieNode());
      cur = cur.ch.get(c);
    }
    cur.order = this.counter++;
  }
  autocomplete(prefix) {
    let cur = this.root;
    for (const c of prefix) {
      if (!cur.ch.has(c)) return [];
      cur = cur.ch.get(c);
    }
    const found = [];
    const dfs = (n, buf) => {
      if (n.order >= 0) found.push([n.order, buf]);
      for (const c of [...n.ch.keys()].sort()) dfs(n.ch.get(c), buf + c);
    };
    dfs(cur, prefix);
    found.sort((a, b) => a[0] - b[0]);
    return found.map(([, w]) => w);
  }
}

const t = new Trie();
for (const w of ["dog", "deer", "deal"]) t.insert(w);
console.log(t.autocomplete("de"));
