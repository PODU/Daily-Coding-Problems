// Merge k sorted linked lists using a binary min-heap of list heads.
// Time: O(N log k), Space: O(k) for the heap.
'use strict';

class ListNode {
  constructor(val) { this.val = val; this.next = null; }
}

function buildList(arr) {
  const dummy = new ListNode(0);
  let cur = dummy;
  for (const x of arr) { cur.next = new ListNode(x); cur = cur.next; }
  return dummy.next;
}

class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
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
        let s = i, l = 2 * i + 1, r = 2 * i + 2;
        if (l < n && a[l].val < a[s].val) s = l;
        if (r < n && a[r].val < a[s].val) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]];
        i = s;
      }
    }
    return top;
  }
}

function mergeKLists(lists) {
  const heap = new MinHeap();
  for (const n of lists) if (n) heap.push(n);
  const dummy = new ListNode(0);
  let tail = dummy;
  while (heap.size()) {
    const node = heap.pop();
    tail.next = node; tail = node;
    if (node.next) heap.push(node.next);
  }
  return dummy.next;
}

function main() {
  const lists = [buildList([1, 4, 5]), buildList([1, 3, 4]), buildList([2, 6])];
  const merged = mergeKLists(lists);
  const out = [];
  for (let n = merged; n; n = n.next) out.push(n.val);
  console.log(out.join(' '));
}

main();
