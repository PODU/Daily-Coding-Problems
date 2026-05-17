// Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
// Post-order recursion. O(n) time, O(h) recursion stack.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function prune(root) {
  if (root === null) return null;
  root.left = prune(root.left);
  root.right = prune(root.right);
  if (root.val === 0 && root.left === null && root.right === null) return null;
  return root;
}

function levelOrder(root) {
  const q = [root];
  const out = [];
  while (q.length) {
    const n = q.shift();
    if (n === null) { out.push("null"); continue; }
    out.push(String(n.val));
    q.push(n.left);
    q.push(n.right);
  }
  while (out.length && out[out.length - 1] === "null") out.pop();
  return "[" + out.join(", ") + "]";
}

const root = new Node(0,
  new Node(1),
  new Node(0,
    new Node(1, new Node(0), new Node(0)),
    new Node(0)));

console.log(levelOrder(prune(root))); // [0, 1, 0, null, null, 1]
