// Evaluate arithmetic expression tree: recurse, combining children by operator
// at each internal node; leaves are integers. Time O(n), Space O(h) recursion.
"use strict";

function leaf(val) {
  return { isLeaf: true, val };
}
function op(o, left, right) {
  return { isLeaf: false, op: o, left, right };
}

function evaluate(node) {
  if (node.isLeaf) return node.val;
  const a = evaluate(node.left), b = evaluate(node.right);
  switch (node.op) {
    case "+": return a + b;
    case "-": return a - b;
    case "*": return a * b;
    case "/": return Math.trunc(a / b);
  }
}

const left = op("+", leaf(3), leaf(2));
const right = op("+", leaf(4), leaf(5));
const root = op("*", left, right);
console.log(evaluate(root));
