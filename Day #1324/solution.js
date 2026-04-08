// Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
// update O(log n), query O(log n). 1-indexed internally over fixed size 24.

class Subscribers {
  constructor() { this.tree = new Array(25).fill(0); }
  update(hour, value) {
    for (let i = hour + 1; i <= 24; i += i & (-i)) this.tree[i] += value;
  }
  prefix(hour) {
    let s = 0;
    for (let i = hour + 1; i > 0; i -= i & (-i)) s += this.tree[i];
    return s;
  }
  query(start, end) {
    return this.prefix(end) - (start > 0 ? this.prefix(start - 1) : 0);
  }
}

const s = new Subscribers();
s.update(2, 10);
s.update(5, 3);
s.update(5, 7);
console.log(s.query(2, 5));  // 20
console.log(s.query(0, 23)); // 20
console.log(s.query(3, 4));  // 0
