// Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function prune(root) {
  if (root === null) return null;
  root.left = prune(root.left);
  root.right = prune(root.right);
  if (root.val === 0 && root.left === null && root.right === null) return null;
  return root;
}

function preorder(r, out) {
  if (r === null) return;
  out.push(r.val);
  preorder(r.left, out);
  preorder(r.right, out);
}

const root = new Node(
  0,
  new Node(1),
  new Node(0, new Node(1, new Node(0), new Node(0)), new Node(0))
);
const pruned = prune(root);
const out = [];
preorder(pruned, out);
console.log("preorder:", out.join(" ")); // 0 1 0 1
