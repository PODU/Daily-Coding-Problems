// Day 1625: Inorder successor in BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is left child. O(h).
class Node {
  constructor(val) {
    this.val = val;
    this.left = this.right = this.parent = null;
  }
}

function inorderSuccessor(node) {
  if (!node) return null;
  if (node.right) {
    let cur = node.right;
    while (cur.left) cur = cur.left;
    return cur;
  }
  let cur = node;
  while (cur.parent && cur.parent.right === cur) cur = cur.parent;
  return cur.parent;
}

function insert(root, v) {
  if (!root) return new Node(v);
  let cur = root;
  while (true) {
    if (v < cur.val) {
      if (!cur.left) { cur.left = new Node(v); cur.left.parent = cur; return root; }
      cur = cur.left;
    } else {
      if (!cur.right) { cur.right = new Node(v); cur.right.parent = cur; return root; }
      cur = cur.right;
    }
  }
}

function find(root, v) {
  while (root && root.val !== v) root = v < root.val ? root.left : root.right;
  return root;
}

let root = null;
for (const v of [10, 5, 30, 22, 35]) root = insert(root, v);
const s = inorderSuccessor(find(root, 22));
console.log(`The inorder successor of 22 is ${s ? s.val : "None"}.`);
