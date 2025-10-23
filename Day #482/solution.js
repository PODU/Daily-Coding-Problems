// Day 482: BST range sum [a,b] inclusive via DFS with pruning.
// Skip left subtree if node<a, skip right if node>b. Time O(n) worst, O(k+h) typical; Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function rangeSum(root, a, b) {
  if (!root) return 0;
  if (root.val < a) return rangeSum(root.right, a, b);
  if (root.val > b) return rangeSum(root.left, a, b);
  return root.val + rangeSum(root.left, a, b) + rangeSum(root.right, a, b);
}

const root = new Node(5,
  new Node(3, new Node(2), new Node(4)),
  new Node(8, new Node(6), new Node(10)));
console.log(rangeSum(root, 4, 9)); // 23
