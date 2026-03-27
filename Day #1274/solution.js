// Day 1274: Prune a 0/1 binary tree, removing every subtree that contains only 0s.
// Post-order recursion: a node survives iff it is 1 or has a surviving child. O(n).
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

function serialize(node) {
  if (node === null) return "null";
  return `${node.val}(${serialize(node.left)},${serialize(node.right)})`;
}

let root = new Node(0, new Node(1), new Node(0, new Node(1, new Node(0), new Node(0)), new Node(0)));
root = prune(root);
// Pruned tree: 0(1(null,null),0(1(null,null),null))
console.log(serialize(root));
