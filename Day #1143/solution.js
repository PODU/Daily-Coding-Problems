// Day 1143: Merge k sorted linked lists.
// Min-heap of list heads. Time O(N log k), Space O(k).
class Node {
  constructor(val) { this.val = val; this.next = null; }
}

class MinHeap {
  constructor() { this.h = []; }
  push(n) {
    this.h.push(n);
    let i = this.h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.h[p].val <= this.h[i].val) break;
      [this.h[p], this.h[i]] = [this.h[i], this.h[p]];
      i = p;
    }
  }
  pop() {
    const top = this.h[0], last = this.h.pop();
    if (this.h.length) {
      this.h[0] = last;
      let i = 0;
      const n = this.h.length;
      while (true) {
        let s = i, l = 2 * i + 1, r = 2 * i + 2;
        if (l < n && this.h[l].val < this.h[s].val) s = l;
        if (r < n && this.h[r].val < this.h[s].val) s = r;
        if (s === i) break;
        [this.h[s], this.h[i]] = [this.h[i], this.h[s]];
        i = s;
      }
    }
    return top;
  }
  get size() { return this.h.length; }
}

function mergeK(lists) {
  const pq = new MinHeap();
  for (const l of lists) if (l) pq.push(l);
  const dummy = new Node(0);
  let tail = dummy;
  while (pq.size) {
    const n = pq.pop();
    tail.next = n; tail = n;
    if (n.next) pq.push(n.next);
  }
  return dummy.next;
}

function build(vals) {
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) { t.next = new Node(x); t = t.next; }
  return dummy.next;
}

const lists = [build([1, 4, 7]), build([2, 5, 8]), build([3, 6, 9])];
const out = [];
for (let n = mergeK(lists); n; n = n.next) out.push(n.val);
console.log(out.join(" -> ")); // 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
