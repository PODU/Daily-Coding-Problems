// Second largest in BST: walk right to largest; second largest is parent of
// largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
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

function secondLargest(root) {
  let parent = null;
  let cur = root;
  while (cur.right) {
    parent = cur;
    cur = cur.right;
  }
  if (cur.left) {
    cur = cur.left;
    while (cur.right) cur = cur.right;
    return cur.val;
  }
  return parent.val;
}

let root = null;
for (const v of [5, 3, 8, 2, 4, 7, 9]) root = insert(root, v);
console.log(secondLargest(root));
