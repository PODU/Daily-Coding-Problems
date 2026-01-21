// Day 936: Sum of BST values within inclusive range [a,b], pruning branches out of range.
// Time O(n) worst, O(h + k) typical, Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function rangeSum(n, a, b) {
  if (n === null) return 0;
  if (n.val < a) return rangeSum(n.right, a, b);
  if (n.val > b) return rangeSum(n.left, a, b);
  return n.val + rangeSum(n.left, a, b) + rangeSum(n.right, a, b);
}

const root = new Node(5,
  new Node(3, new Node(2), new Node(4)),
  new Node(8, new Node(6), new Node(10)));
console.log(rangeSum(root, 4, 9)); // 23
