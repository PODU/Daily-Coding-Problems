// DFS with BST pruning: skip left if val<a, skip right if val>b. O(n) worst.
// O(n) time worst, O(h) space (recursion).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function rangeSumBST(node, a, b) {
  if (!node) return 0;
  if (node.val < a) return rangeSumBST(node.right, a, b);
  if (node.val > b) return rangeSumBST(node.left, a, b);
  return node.val + rangeSumBST(node.left, a, b) + rangeSumBST(node.right, a, b);
}

const root = new Node(
  5,
  new Node(3, new Node(2), new Node(4)),
  new Node(8, new Node(6), new Node(10))
);
console.log(rangeSumBST(root, 4, 9));
