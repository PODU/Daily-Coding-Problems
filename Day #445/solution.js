// Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
// Postorder recursion, O(n) time, O(h) space.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function prune(node) {
  if (node === null) return null;
  node.left = prune(node.left);
  node.right = prune(node.right);
  if (node.val === 0 && node.left === null && node.right === null) return null;
  return node;
}

function show(n, prefix = "", tag = "") {
  if (n === null) return;
  console.log(prefix + tag + n.val);
  show(n.left, prefix + "  ", "L-- ");
  show(n.right, prefix + "  ", "R-- ");
}

let root = new Node(0,
  new Node(1),
  new Node(0,
    new Node(1, new Node(0), new Node(0)),
    new Node(0)));
root = prune(root);
show(root);
// 0
//   L-- 1
//   R-- 0
//     L-- 1
