// Day 133: Inorder successor in a BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
    this.parent = null;
  }
}

function successor(node) {
  if (!node) return null;
  if (node.right) {
    let c = node.right;
    while (c.left) c = c.left;
    return c;
  }
  let p = node.parent;
  while (p && node === p.right) {
    node = p;
    p = p.parent;
  }
  return p;
}

function attach(parent, child) {
  if (child) child.parent = parent;
  return child;
}

const root = new Node(10);
root.left = attach(root, new Node(5));
root.right = attach(root, new Node(30));
const n22 = new Node(22), n35 = new Node(35);
root.right.left = attach(root.right, n22);
root.right.right = attach(root.right, n35);

const s = successor(n22);
console.log(s ? s.val : "null"); // 30
