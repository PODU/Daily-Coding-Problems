// Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
// Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function isBST(node, lo = -Infinity, hi = Infinity) {
  if (!node) return true;
  if (node.val < lo || node.val > hi) return false;
  return isBST(node.left, lo, node.val) && isBST(node.right, node.val, hi);
}

const a = new Node(5, new Node(3, new Node(2), new Node(4)), new Node(8));
console.log(isBST(a)); // true

const b = new Node(5, new Node(3, null, new Node(6)), new Node(8)); // invalid
console.log(isBST(b)); // false
