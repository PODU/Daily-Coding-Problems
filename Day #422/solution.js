// Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
// Then print merged tree by level-order traversal (skipping null children).
class Node {
  constructor(v, l = null, r = null) {
    this.val = v;
    this.left = l;
    this.right = r;
  }
}

function merge(a, b) {
  if (!a) return b;
  if (!b) return a;
  const n = new Node(a.val + b.val);
  n.left = merge(a.left, b.left);
  n.right = merge(a.right, b.right);
  return n;
}

const t1 = new Node(1, new Node(3, new Node(5)), new Node(2));
const t2 = new Node(2, new Node(1, null, new Node(4)), new Node(3, null, new Node(7)));
const m = merge(t1, t2);

const out = [], q = [m];
while (q.length) {
  const c = q.shift();
  out.push(c.val);
  if (c.left) q.push(c.left);
  if (c.right) q.push(c.right);
}
console.log(out.join(" "));
