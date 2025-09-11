// Prune to full binary tree: post-order DFS; a node with exactly one child is
// replaced by that child. Returns new root. Time O(n), Space O(h) recursion.
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function prune(root) {
  if (root === null) return null;
  root.left = prune(root.left);
  root.right = prune(root.right);
  if (root.left && !root.right) return root.left;
  if (root.right && !root.left) return root.right;
  return root;
}

function preorder(node, out) {
  if (node === null) return;
  out.push(node.val);
  preorder(node.left, out);
  preorder(node.right, out);
}

const root0 = new Node(0);
root0.left = new Node(1);
root0.right = new Node(2);
root0.left.left = new Node(3);
root0.left.left.right = new Node(5);
root0.right.right = new Node(4);
root0.right.right.left = new Node(6);
root0.right.right.right = new Node(7);

const root = prune(root0);
const pre = [];
preorder(root, pre);
console.log("Preorder after pruning: " + pre.join(" "));
