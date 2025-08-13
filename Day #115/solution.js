// Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function same(a, b) {
  if (a === null && b === null) return true;
  if (a === null || b === null || a.val !== b.val) return false;
  return same(a.left, b.left) && same(a.right, b.right);
}

function isSubtree(s, t) {
  if (t === null) return true;
  if (s === null) return false;
  if (same(s, t)) return true;
  return isSubtree(s.left, t) || isSubtree(s.right, t);
}

const s = new Node(3, new Node(4, new Node(1), new Node(2)), new Node(5));
const t = new Node(4, new Node(1), new Node(2));
const u = new Node(4, new Node(0));
console.log(isSubtree(s, t)); // true
console.log(isSubtree(s, u)); // false
