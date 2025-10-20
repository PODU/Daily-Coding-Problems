// Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
// Time: O(n), Space: O(h) recursion.
class Node {
  constructor(val) {
    this.val = val;
    this.children = [];
  }
}

function mirror(a, b) {
  if (a === null && b === null) return true;
  if (a === null || b === null) return false;
  if (a.val !== b.val) return false;
  if (a.children.length !== b.children.length) return false;
  const n = a.children.length;
  for (let i = 0; i < n; i++)
    if (!mirror(a.children[i], b.children[n - 1 - i])) return false;
  return true;
}

function isSymmetric(root) {
  if (root === null) return true;
  return mirror(root, root);
}

const root = new Node(4);
const l3 = new Node(3), m5 = new Node(5), r3 = new Node(3);
root.children = [l3, m5, r3];
l3.children = [new Node(9)];
r3.children = [new Node(9)];
console.log(isSymmetric(root) ? "True" : "False");
