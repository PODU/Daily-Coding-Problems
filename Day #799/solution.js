// Day 799: PrefixMapSum - trie where each node stores sum of values below it.
// insert overwrites via delta (new-old) propagated along the path.
// insert O(L), sum O(L). Space O(total chars).

class Node {
  constructor() {
    this.total = 0;
    this.child = new Map();
  }
}

class PrefixMapSum {
  constructor() {
    this.root = new Node();
    this.vals = new Map();
  }

  insert(key, value) {
    const delta = value - (this.vals.get(key) || 0);
    this.vals.set(key, value);
    let cur = this.root;
    cur.total += delta;
    for (const c of key) {
      if (!cur.child.has(c)) cur.child.set(c, new Node());
      cur = cur.child.get(c);
      cur.total += delta;
    }
  }

  sum(prefix) {
    let cur = this.root;
    for (const c of prefix) {
      if (!cur.child.has(c)) return 0;
      cur = cur.child.get(c);
    }
    return cur.total;
  }
}

const m = new PrefixMapSum();
m.insert("columnar", 3);
console.log(m.sum("col")); // 3
m.insert("column", 2);
console.log(m.sum("col")); // 5
