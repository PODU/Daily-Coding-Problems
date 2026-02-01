// Day 1001: Validate a binary search tree.
// Recurse carrying an allowed [low, high] range; left key <= root <= right key.
// O(n) time, O(h) space.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function isBst(node, low = -Infinity, high = Infinity) {
  if (node === null) return true;
  if (node.val < low || node.val > high) return false;
  return isBst(node.left, low, node.val) && isBst(node.right, node.val, high);
}

const valid = new Node(5, new Node(3, new Node(2), new Node(4)), new Node(8, new Node(6), new Node(9)));
const invalid = new Node(5, new Node(6), new Node(8));
console.log(isBst(valid)); // true
console.log(isBst(invalid)); // false
