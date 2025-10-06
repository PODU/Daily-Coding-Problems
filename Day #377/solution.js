// Sliding window median via two heaps with lazy deletion.
// Time: O(n log k), Space: O(k).
class Heap {
  constructor(cmp) { this.a = []; this.cmp = cmp; }
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(x) {
    const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0; const n = a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < n && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < n && this.cmp(a[r], a[s]) < 0) s = r;
        if (s !== i) { [a[i], a[s]] = [a[s], a[i]]; i = s; } else break;
      }
    }
    return top;
  }
}

class DualHeap {
  constructor() {
    this.small = new Heap((a, b) => b - a); // max-heap
    this.large = new Heap((a, b) => a - b); // min-heap
    this.delayed = new Map();
    this.ss = 0; this.ls = 0;
  }
  prune(heap) {
    while (heap.size()) {
      const num = heap.peek();
      const cnt = this.delayed.get(num) || 0;
      if (cnt > 0) {
        if (cnt === 1) this.delayed.delete(num); else this.delayed.set(num, cnt - 1);
        heap.pop();
      } else break;
    }
  }
  balance() {
    if (this.ss > this.ls + 1) { this.large.push(this.small.pop()); this.ss--; this.ls++; this.prune(this.small); }
    else if (this.ss < this.ls) { this.small.push(this.large.pop()); this.ls--; this.ss++; this.prune(this.large); }
  }
  insert(num) {
    if (!this.small.size() || num <= this.small.peek()) { this.small.push(num); this.ss++; }
    else { this.large.push(num); this.ls++; }
    this.balance();
  }
  erase(num) {
    this.delayed.set(num, (this.delayed.get(num) || 0) + 1);
    if (num <= this.small.peek()) { this.ss--; if (num === this.small.peek()) this.prune(this.small); }
    else { this.ls--; if (num === this.large.peek()) this.prune(this.large); }
    this.balance();
  }
  median(k) {
    if (k % 2 === 1) return this.small.peek();
    return (this.small.peek() + this.large.peek()) / 2;
  }
}

const arr = [-1, 5, 13, 8, 2, 3, 3, 1];
const k = 3;
const dh = new DualHeap();
for (let i = 0; i < k; i++) dh.insert(arr[i]);
const n = arr.length;
for (let i = k; i <= n; i++) {
  const win = arr.slice(i - k, i);
  console.log(`${dh.median(k)} <- median of [${win.join(", ")}]`);
  if (i < n) { dh.insert(arr[i]); dh.erase(arr[i - k]); }
}
