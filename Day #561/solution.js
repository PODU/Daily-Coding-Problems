// Sorted array -> height-balanced BST: recurse on mid=(lo+hi>>1) as root.
// Time: O(N), Space: O(N) for nodes + O(log N) recursion.
'use strict';

class TreeNode {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function build(a, lo, hi) {
  if (lo > hi) return null;
  const mid = (lo + hi) >> 1;
  const root = new TreeNode(a[mid]);
  root.left = build(a, lo, mid - 1);
  root.right = build(a, mid + 1, hi);
  return root;
}

function main() {
  const a = [1, 2, 3, 4, 5, 6, 7];
  const root = build(a, 0, a.length - 1);
  const out = [];
  const q = [];
  if (root) q.push(root);
  while (q.length) {
    const n = q.shift();
    out.push(n.val);
    if (n.left) q.push(n.left);
    if (n.right) q.push(n.right);
  }
  console.log(out.join(' '));
}

main();
