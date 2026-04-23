// Day 1411: Check if tree t is a subtree of tree s.
// Approach: recursive DFS — for each node of s try exact-match with t. O(|s|*|t|) time, O(h) space.

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
console.log(isSubtree(s, t) ? "true" : "false");
