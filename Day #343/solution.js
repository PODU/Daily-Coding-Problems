// Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function rangeSum(root, a, b) {
  if (!root) return 0;
  let s = 0;
  if (root.val >= a && root.val <= b) s += root.val;
  if (root.val > a) s += rangeSum(root.left, a, b);
  if (root.val < b) s += rangeSum(root.right, a, b);
  return s;
}

function main() {
  const root = new Node(5);
  root.left = new Node(3);
  root.left.left = new Node(2);
  root.left.right = new Node(4);
  root.right = new Node(8);
  root.right.left = new Node(6);
  root.right.right = new Node(10);
  console.log(rangeSum(root, 4, 9));
}

main();
