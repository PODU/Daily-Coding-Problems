// Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
// Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function height(node) {
  if (node === null) return 0;
  const lh = height(node.left);
  if (lh === -1) return -1;
  const rh = height(node.right);
  if (rh === -1) return -1;
  if (Math.abs(lh - rh) > 1) return -1;
  return Math.max(lh, rh) + 1;
}

function isBalanced(root) {
  return height(root) !== -1;
}

// Balanced tree
const a = new Node(1, new Node(2, new Node(4)), new Node(3));
// Unbalanced left-leaning chain 1 -> 2 -> 3 -> 4
const b = new Node(1, new Node(2, new Node(3, new Node(4))));

console.log(isBalanced(a) ? "true" : "false");
console.log(isBalanced(b) ? "true" : "false");
