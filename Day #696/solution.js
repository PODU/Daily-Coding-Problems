// Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
// Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).
class Fenwick {
  constructor(n) { this.n = n; this.t = new Array(n + 1).fill(0); }
  add(i, v) { for (i++; i <= this.n; i += i & -i) this.t[i] += v; }
  pref(i) { let s = 0; for (i++; i > 0; i -= i & -i) s += this.t[i]; return s; }
  range(l, r) { return this.pref(r) - (l ? this.pref(l - 1) : 0); }
}

class Subscribers {
  constructor() { this.f = new Fenwick(24); }
  update(hour, value) { this.f.add(hour, value); }
  query(start, end) { return this.f.range(start, end); }
}

const s = new Subscribers();
s.update(3, 10); s.update(5, 7); s.update(10, 4);
console.log(s.query(3, 10)); // 21
console.log(s.query(0, 4));  // 10
s.update(3, 5);
console.log(s.query(3, 10)); // 26
