// Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
// Recursively compare children in mirrored order. Time: O(N), Space: O(height).
class Node {
  constructor(val, children = []) { this.val = val; this.children = children; }
}

function mirror(a, b) {
  if (!a && !b) return true;
  if (!a || !b) return false;
  if (a.val !== b.val || a.children.length !== b.children.length) return false;
  const n = a.children.length;
  for (let i = 0; i < n; i++)
    if (!mirror(a.children[i], b.children[n - 1 - i])) return false;
  return true;
}

function isSymmetric(root) {
  if (!root) return true;
  const n = root.children.length;
  for (let i = 0; i < Math.floor(n / 2); i++)
    if (!mirror(root.children[i], root.children[n - 1 - i])) return false;
  return true;
}

const root = new Node(4, [
  new Node(3, [new Node(9)]),
  new Node(5),
  new Node(3, [new Node(9)]),
]);
console.log(isSymmetric(root)); // true
