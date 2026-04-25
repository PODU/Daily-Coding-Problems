// Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
// Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.

class Node {
  constructor(op, val, left = null, right = null) {
    this.op = op; // operator string for internal nodes, null for leaves
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

const leaf = (v) => new Node(null, v);

function evalTree(n) {
  if (n.left === null && n.right === null) return n.val;
  const a = evalTree(n.left), b = evalTree(n.right);
  switch (n.op) {
    case "+": return a + b;
    case "-": return a - b;
    case "*": return a * b;
    default:  return a / b;
  }
}

const root = new Node("*", 0,
  new Node("+", 0, leaf(3), leaf(2)),
  new Node("+", 0, leaf(4), leaf(5)));
console.log(evalTree(root)); // 45
