// Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
// mid = (lo+hi)>>1 (lower-middle). Time O(n), Space O(log n) recursion.
'use strict';

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
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

function main() {
  const a = [-10, -3, 0, 5, 9];
  const root = build(a, 0, a.length - 1);
  const out = [];
  preorder(root, out);
  console.log(out.join(' '));
}

main();
