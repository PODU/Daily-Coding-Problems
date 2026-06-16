// Day 1676: PrefixMapSum via prefix-sum map + delta on overwrite.
// insert/sum both O(key length). Space O(total chars).
class PrefixMapSum {
  constructor() {
    this.total = new Map(); // prefix -> cumulative sum
    this.vals = new Map();  // key -> value
  }
  insert(key, value) {
    const delta = value - (this.vals.get(key) || 0);
    this.vals.set(key, value);
    let prefix = "";
    for (const ch of key) {
      prefix += ch;
      this.total.set(prefix, (this.total.get(prefix) || 0) + delta);
    }
  }
  sum(prefix) {
    return this.total.get(prefix) || 0;
  }
}

const m = new PrefixMapSum();
m.insert("columnar", 3);
console.log(m.sum("col")); // 3
m.insert("column", 2);
console.log(m.sum("col")); // 5
