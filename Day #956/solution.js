// Day 956: merge k sorted singly linked lists using a min-heap.
// Time O(N log k), Space O(k) where N = total nodes.

class ListNode {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

// Minimal binary min-heap keyed on node.val.
class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(n) {
    const a = this.a; a.push(n);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p].val <= a[i].val) break;
      [a[p], a[i]] = [a[i], a[p]]; i = p;
    }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l].val < a[s].val) s = l;
        if (r < a.length && a[r].val < a[s].val) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]]; i = s;
      }
    }
    return top;
  }
}

function mergeK(lists) {
  const h = new MinHeap();
  for (const l of lists) if (l) h.push(l);
  const dummy = new ListNode(0);
  let tail = dummy;
  while (h.size()) {
    const n = h.pop();
    tail.next = n; tail = n;
    if (n.next) h.push(n.next);
  }
  return dummy.next;
}

function build(vals) {
  const dummy = new ListNode(0);
  let t = dummy;
  for (const x of vals) { t.next = new ListNode(x); t = t.next; }
  return dummy.next;
}

const lists = [build([1, 4, 5]), build([1, 3, 4]), build([2, 6])];
let m = mergeK(lists);
const out = [];
while (m) { out.push(m.val); m = m.next; }
console.log(out.join(" ")); // 1 1 2 3 4 4 5 6
