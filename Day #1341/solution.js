// Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
// Time O(n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function height(root) {
  if (root === null) return 0;
  const l = height(root.left);
  if (l === -1) return -1;
  const r = height(root.right);
  if (r === -1) return -1;
  if (Math.abs(l - r) > 1) return -1;
  return Math.max(l, r) + 1;
}

function isBalanced(root) {
  return height(root) !== -1;
}

// Balanced tree [1,2,3,4,5,null,6]
const a = new Node(1, new Node(2, new Node(4), new Node(5)), new Node(3, null, new Node(6)));
console.log(`Balanced: ${isBalanced(a)}`);

// Skewed tree 1 -> 2 -> 3
const b = new Node(1, new Node(2, new Node(3)));
console.log(`Balanced: ${isBalanced(b)}`);
