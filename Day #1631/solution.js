// Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
'use strict';

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function isValid(n, lo, hi) {
  if (n === null) return true;
  if (n.val < lo || n.val > hi) return false;
  return isValid(n.left, lo, n.val) && isValid(n.right, n.val, hi);
}

function validate(root) {
  return isValid(root, -Infinity, Infinity);
}

function main() {
  // Valid BST: root 5, left 3 (2,4), right 8 (6,9)
  const root = new Node(5);
  root.left = new Node(3);
  root.left.left = new Node(2);
  root.left.right = new Node(4);
  root.right = new Node(8);
  root.right.left = new Node(6);
  root.right.right = new Node(9);

  // Invalid: root 5, left child 6
  const bad = new Node(5);
  bad.left = new Node(6);

  console.log(validate(root) ? 'true' : 'false');
  console.log(validate(bad) ? 'true' : 'false');
}

main();
