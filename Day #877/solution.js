// Subtree check: for each node of s, test identical-tree with t. Time O(m*n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function sameTree(a, b) {
  if (a === null && b === null) return true;
  if (a === null || b === null) return false;
  return a.val === b.val && sameTree(a.left, b.left) && sameTree(a.right, b.right);
}

function isSubtree(s, t) {
  if (s === null) return false;
  if (sameTree(s, t)) return true;
  return isSubtree(s.left, t) || isSubtree(s.right, t);
}

const s = new Node(3, new Node(4, new Node(1), new Node(2)), new Node(5));
const t = new Node(4, new Node(1), new Node(2));
console.log(isSubtree(s, t));
