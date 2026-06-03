// Day 1606: Check if a binary tree is height-balanced.
// Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function isBalanced(root) {
  function check(node) { // returns height, or -1 if unbalanced
    if (!node) return 0;
    const l = check(node.left);
    if (l === -1) return -1;
    const r = check(node.right);
    if (r === -1) return -1;
    if (Math.abs(l - r) > 1) return -1;
    return Math.max(l, r) + 1;
  }
  return check(root) !== -1;
}

const root = new Node(1, new Node(2, new Node(4)), new Node(3));
console.log(isBalanced(root) ? "true" : "false"); // true
