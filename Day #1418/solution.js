// Day 1418: inorder successor of a BST node using parent pointers.
// Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
    this.parent = null;
  }
}

function successor(node) {
  if (node.right) {
    let cur = node.right;
    while (cur.left) cur = cur.left;
    return cur;
  }
  let cur = node;
  while (cur.parent && cur === cur.parent.right) cur = cur.parent;
  return cur.parent;
}

function attach(parent, child) {
  if (child) child.parent = parent;
  return child;
}

const root = new Node(10);
root.left = attach(root, new Node(5));
root.right = attach(root, new Node(30));
const n22 = new Node(22);
root.right.left = attach(root.right, n22);
root.right.right = attach(root.right, new Node(35));

const s = successor(n22);
console.log(s ? s.val : "null"); // 30
