// Subtree check: at each node of s, test sameTree(node, t).
// Time: O(|s|*|t|) worst case. Space: O(height). (Optimal O(|s|+|t|) via
// tree serialization + KMP substring search; recursive version implemented.)
'use strict';

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

// s:        3
//          / \
//         4   5
//        / \
//       1   2
const s = new Node(3, new Node(4, new Node(1), new Node(2)), new Node(5));
const t = new Node(4, new Node(1), new Node(2));
const t2 = new Node(4, new Node(1), new Node(0));

console.log(isSubtree(s, t));
console.log(isSubtree(s, t2));
