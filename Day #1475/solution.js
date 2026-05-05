// Day 1475: Autocomplete via trie. Walk to prefix node, collect subtree words.
// Build O(total chars); query O(len(prefix) + matches). Space O(total chars).

class Trie {
  constructor() { this.root = {}; }
  insert(word, order) {
    let node = this.root;
    for (const ch of word) node = node[ch] || (node[ch] = {});
    node['#'] = [word, order];
  }
  startsWith(prefix) {
    let node = this.root;
    for (const ch of prefix) {
      if (!(ch in node)) return [];
      node = node[ch];
    }
    const out = [];
    const dfs = (n) => {
      if ('#' in n) out.push(n['#']);
      for (const k of Object.keys(n)) if (k !== '#') dfs(n[k]);
    };
    dfs(node);
    out.sort((a, b) => a[1] - b[1]);
    return out.map((t) => t[0]);
  }
}

const t = new Trie();
["dog", "deer", "deal"].forEach((w, i) => t.insert(w, i));
console.log(t.startsWith("de")); // [ 'deer', 'deal' ]
