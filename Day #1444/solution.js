// Day 1444: Convert a binary tree to a full binary tree by removing every node
// with exactly one child (its single child is promoted up).
// Post-order recursion. Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function toFull(root) {
  if (root === null) return null;
  root.left = toFull(root.left);
  root.right = toFull(root.right);
  if (root.left && !root.right) return root.left;
  if (root.right && !root.left) return root.right;
  return root;
}

function preorder(r, out) {
  if (r === null) return;
  out.push(r.val);
  preorder(r.left, out);
  preorder(r.right, out);
}

const n0 = new Node(0, new Node(1), new Node(2));
n0.left.left = new Node(3);
n0.left.left.right = new Node(5);
n0.right.right = new Node(4);
n0.right.right.left = new Node(6);
n0.right.right.right = new Node(7);

const full = toFull(n0);
const out = [];
preorder(full, out);
console.log("Preorder of full tree:", out.join(" ")); // 0 5 4 6 7
