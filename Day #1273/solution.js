// Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
// Trie storing accumulated sums; insert applies the delta vs the old value.
// insert/sum are O(key length).
class TrieNode {
  constructor() { this.sum = 0; this.next = new Map(); }
}

class PrefixMapSum {
  constructor() { this.root = new TrieNode(); this.vals = new Map(); }

  insert(key, value) {
    const delta = value - (this.vals.get(key) || 0);
    this.vals.set(key, value);
    let node = this.root;
    for (const c of key) {
      if (!node.next.has(c)) node.next.set(c, new TrieNode());
      node = node.next.get(c);
      node.sum += delta;
    }
  }

  sum(prefix) {
    let node = this.root;
    for (const c of prefix) {
      if (!node.next.has(c)) return 0;
      node = node.next.get(c);
    }
    return node.sum;
  }
}

const mapsum = new PrefixMapSum();
mapsum.insert("columnar", 3);
console.log(mapsum.sum("col")); // 3
mapsum.insert("column", 2);
console.log(mapsum.sum("col")); // 5
