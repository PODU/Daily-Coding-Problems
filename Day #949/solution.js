// Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
// Build O(total chars); query O(|s| + matches). Insertion order preserved.

class Node {
  constructor() {
    this.next = new Map();
    this.ids = [];
  }
}

class Trie {
  constructor() {
    this.root = new Node();
    this.words = [];
  }
  insert(w) {
    const idx = this.words.length;
    this.words.push(w);
    let cur = this.root;
    for (const c of w) {
      if (!cur.next.has(c)) cur.next.set(c, new Node());
      cur = cur.next.get(c);
      cur.ids.push(idx);
    }
  }
  withPrefix(s) {
    let cur = this.root;
    for (const c of s) {
      if (!cur.next.has(c)) return [];
      cur = cur.next.get(c);
    }
    return cur.ids.map((i) => this.words[i]);
  }
}

const t = new Trie();
for (const w of ["dog", "deer", "deal"]) t.insert(w);
console.log("[" + t.withPrefix("de").join(", ") + "]"); // [deer, deal]
