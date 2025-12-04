// Day 692: Autocomplete - return all dictionary strings having s as a prefix.
// Approach: Trie. Insert words O(total chars); query walks prefix then DFS to
// collect matches. Query O(|s| + #matches * len).
class Trie {
  constructor() { this.root = {}; }
  insert(word) {
    let node = this.root;
    for (const c of word) node = node[c] || (node[c] = {});
    node["$"] = true;
  }
  autocomplete(s) {
    let node = this.root;
    for (const c of s) { if (!(c in node)) return []; node = node[c]; }
    const out = [];
    const dfs = (n, cur) => {
      if (n["$"]) out.push(cur);
      for (const c of Object.keys(n)) if (c !== "$") dfs(n[c], cur + c);
    };
    dfs(node, s);
    return out;
  }
}

const t = new Trie();
for (const w of ["dog", "deer", "deal"]) t.insert(w);
console.log(t.autocomplete("de").sort());   // [ 'deal', 'deer' ]
