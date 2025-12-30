// Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
// Time: O(n), Space: O(h) recursion.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function merge(a, b) {
  if (a === null) return b;
  if (b === null) return a;
  const n = new Node(a.val + b.val);
  n.left = merge(a.left, b.left);
  n.right = merge(a.right, b.right);
  return n;
}

function preorder(n, out) {
  if (n === null) return;
  out.push(n.val);
  preorder(n.left, out);
  preorder(n.right, out);
}

const t1 = new Node(1, new Node(3, new Node(5)), new Node(2));
const t2 = new Node(2, new Node(1, null, new Node(4)), new Node(3, null, new Node(7)));
const m = merge(t1, t2);
const res = [];
preorder(m, res);
console.log("[" + res.join(", ") + "]");
