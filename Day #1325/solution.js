// Day 1325: Sorted array -> height-balanced BST.
// Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.

class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function build(a, lo, hi) {
  if (lo > hi) return null;
  const mid = (lo + hi) >> 1;
  const root = new Node(a[mid]);
  root.left = build(a, lo, mid - 1);
  root.right = build(a, mid + 1, hi);
  return root;
}

function preorder(node, out) {
  if (!node) return;
  out.push(node.val);
  preorder(node.left, out);
  preorder(node.right, out);
}

const a = [1, 2, 3, 4, 5, 6, 7];
const root = build(a, 0, a.length - 1);
const out = [];
preorder(root, out);
console.log("preorder: [" + out.join(", ") + "]"); // preorder: [4, 2, 1, 3, 6, 5, 7]
