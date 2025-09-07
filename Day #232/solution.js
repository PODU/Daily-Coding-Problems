// PrefixMapSum: Trie where each node stores the running sum of values passing through it.
// On overwrite, propagate delta = new - old. insert/sum both O(key length).
class Node {
  constructor() { this.next = new Map(); this.sum = 0; }
}

class PrefixMapSum {
  constructor() { this.root = new Node(); this.vals = new Map(); }

  insert(key, value) {
    const delta = value - (this.vals.get(key) || 0);
    this.vals.set(key, value);
    let n = this.root;
    for (const c of key) {
      if (!n.next.has(c)) n.next.set(c, new Node());
      n = n.next.get(c);
      n.sum += delta;
    }
  }

  sum(prefix) {
    let n = this.root;
    for (const c of prefix) {
      if (!n.next.has(c)) return 0;
      n = n.next.get(c);
    }
    return n.sum;
  }
}

const mapsum = new PrefixMapSum();
mapsum.insert("columnar", 3);
console.log(mapsum.sum("col")); // 3
mapsum.insert("column", 2);
console.log(mapsum.sum("col")); // 5
