// Merge k sorted linked lists via binary min-heap of current heads. O(N log k) time, O(k) space.

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function build(vals) {
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) {
    t.next = new Node(x);
    t = t.next;
  }
  return dummy.next;
}

class MinHeap {
  constructor() {
    this.a = [];
  }
  push(node) {
    const a = this.a;
    a.push(node);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p].val <= a[i].val) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  }
  pop() {
    const a = this.a;
    const top = a[0];
    const last = a.pop();
    if (a.length) {
      a[0] = last;
      let i = 0;
      const n = a.length;
      while (true) {
        let s = i;
        const l = 2 * i + 1, r = 2 * i + 2;
        if (l < n && a[l].val < a[s].val) s = l;
        if (r < n && a[r].val < a[s].val) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]];
        i = s;
      }
    }
    return top;
  }
  get size() {
    return this.a.length;
  }
}

function mergeK(lists) {
  const heap = new MinHeap();
  for (const l of lists) if (l) heap.push(l);
  const dummy = new Node(0);
  let tail = dummy;
  while (heap.size) {
    const n = heap.pop();
    tail.next = n;
    tail = n;
    if (n.next) heap.push(n.next);
  }
  return dummy.next;
}

function main() {
  const lists = [build([1, 4, 5]), build([1, 3, 4]), build([2, 6])];
  let m = mergeK(lists);
  const out = [];
  for (let p = m; p; p = p.next) out.push(p.val);
  console.log(out.join(" "));
}

main();
