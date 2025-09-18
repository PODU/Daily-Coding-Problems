// Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
// Time O(n), Space O(log n) recursion.
class Node {
  constructor(val) { this.val = val; this.l = null; this.r = null; }
}

function build(a, lo, hi) {
  if (lo > hi) return null;
  const mid = Math.floor((lo + hi) / 2);
  const n = new Node(a[mid]);
  n.l = build(a, lo, mid - 1);
  n.r = build(a, mid + 1, hi);
  return n;
}

const a = [-10, -3, 0, 5, 9];
const root = build(a, 0, a.length - 1);
const order = [];
const q = [root];
while (q.length) {
  const n = q.shift();
  if (n === null) continue;
  order.push(n.val);
  q.push(n.l); q.push(n.r);
}
console.log("[" + order.join(", ") + "]");
