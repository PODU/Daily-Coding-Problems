// Convert binary tree to FULL binary tree by removing nodes with exactly one child.
// Post-order recursion: a one-child node is replaced by its processed child.
// Time: O(n), Space: O(h) recursion.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function fullTree(root) {
  if (root === null) return null;
  root.left = fullTree(root.left);
  root.right = fullTree(root.right);
  if (root.left && !root.right) return root.left;
  if (root.right && !root.left) return root.right;
  return root;
}

function preorder(root, out) {
  if (root === null) return;
  out.push(root.val);
  preorder(root.left, out);
  preorder(root.right, out);
}

function main() {
  const root = new Node(0);
  root.left = new Node(1);
  root.right = new Node(2);
  root.left.left = new Node(3);
  root.left.left.right = new Node(5);
  root.right.right = new Node(4);
  root.right.right.left = new Node(6);
  root.right.right.right = new Node(7);

  const result = fullTree(root);
  const out = [];
  preorder(result, out);
  console.log(out.join(" "));
}

main();
