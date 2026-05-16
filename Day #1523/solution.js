// Second largest in BST via parent-walk: find largest; if it has a left subtree,
// answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function insert(root, v) {
  if (root === null) return new Node(v);
  if (v < root.val) root.left = insert(root.left, v);
  else root.right = insert(root.right, v);
  return root;
}

function maxNode(n) {
  while (n.right) n = n.right;
  return n.val;
}

function secondLargest(root) {
  let cur = root, parent = null;
  while (cur.right) {
    parent = cur;
    cur = cur.right;
  }
  if (cur.left) return maxNode(cur.left);
  return parent.val;
}

let root = null;
for (const v of [5, 3, 8, 2, 4, 7, 9]) root = insert(root, v);
console.log(secondLargest(root));
