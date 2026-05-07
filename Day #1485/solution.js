// Day 1485: Determine whether a k-ary tree is symmetric about its root.
// Subtrees mirror iff values match and child i mirrors child (k-1-i).
// Time O(N), Space O(H).

class Node {
  constructor(val, children = []) {
    this.val = val;
    this.children = children;
  }
}

function isMirror(a, b) {
  if (a.val !== b.val || a.children.length !== b.children.length) return false;
  const k = a.children.length;
  for (let i = 0; i < k; ++i)
    if (!isMirror(a.children[i], b.children[k - 1 - i])) return false;
  return true;
}

function isSymmetric(root) {
  return root === null || isMirror(root, root);
}

const tree = new Node(4, [
  new Node(3, [new Node(9)]),
  new Node(5),
  new Node(3, [new Node(9)]),
]);
console.log(isSymmetric(tree)); // true
