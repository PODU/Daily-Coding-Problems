// Day 992: Second largest node in a BST.
// The largest is the rightmost node; the 2nd largest is its left subtree's max,
// or its parent if it has no left child. O(h) time, O(1) extra space.

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function insert(root, val) {
  if (root === null) return new Node(val);
  if (val < root.val) root.left = insert(root.left, val);
  else root.right = insert(root.right, val);
  return root;
}

function secondLargest(root) {
  let cur = root,
    parent = null;
  while (cur.right !== null) {
    parent = cur;
    cur = cur.right;
  }
  if (cur.left !== null) {
    cur = cur.left;
    while (cur.right !== null) cur = cur.right;
    return cur.val;
  }
  return parent.val;
}

let root = null;
for (const v of [5, 3, 8, 2, 4, 7, 9]) root = insert(root, v);
console.log(secondLargest(root)); // 8
