// Day 484: Second largest node in a BST.
// O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
// Time O(h), Space O(1).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function insert(root, v) {
  if (!root) return new Node(v);
  if (v < root.val) root.left = insert(root.left, v);
  else root.right = insert(root.right, v);
  return root;
}

function secondLargest(root) {
  if (!root || (!root.left && !root.right)) return null;
  let cur = root, parent = null;
  while (cur.right) { parent = cur; cur = cur.right; }
  if (cur.left) {
    cur = cur.left;
    while (cur.right) cur = cur.right;
    return cur;
  }
  return parent;
}

let root = null;
for (const v of [5, 3, 8, 2, 4, 7, 10]) root = insert(root, v);
console.log(secondLargest(root).val); // 8
