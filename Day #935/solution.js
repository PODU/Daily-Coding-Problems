// Day 935: Check if a binary tree is height-balanced.
// Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function isBalanced(root) {
  function check(n) {
    if (n === null) return 0;
    const l = check(n.left);
    if (l === -1) return -1;
    const r = check(n.right);
    if (r === -1) return -1;
    if (Math.abs(l - r) > 1) return -1;
    return 1 + Math.max(l, r);
  }
  return check(root) !== -1;
}

const a = new Node(1, new Node(2, new Node(4)), new Node(3));
console.log(isBalanced(a)); // true

const b = new Node(1, new Node(2, new Node(3, new Node(4))));
console.log(isBalanced(b)); // false
