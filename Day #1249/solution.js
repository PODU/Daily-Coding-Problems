// Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function isValid(node, lo, hi) {
  if (node === null) return true;
  if (node.val < lo || node.val > hi) return false;
  return isValid(node.left, lo, node.val) && isValid(node.right, node.val, hi);
}

function isValidBST(root) {
  return isValid(root, -Infinity, Infinity);
}

function main() {
  // Valid tree: root 5, left 3 (2,5), right 8 (5,12)
  const root = new Node(5);
  root.left = new Node(3);
  root.left.left = new Node(2);
  root.left.right = new Node(5);
  root.right = new Node(8);
  root.right.left = new Node(5);
  root.right.right = new Node(12);
  console.log(isValidBST(root) ? "true" : "false");

  // Invalid tree: root 5, left 6 (6 > 5 violates)
  const bad = new Node(5);
  bad.left = new Node(6);
  bad.right = new Node(8);
  console.log(isValidBST(bad) ? "true" : "false");
}

main();
