// Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
// Recursive post-order evaluation. Time O(n), Space O(h).

class Node {
  constructor({ val = null, op = null, left = null, right = null }) {
    this.val = val;
    this.op = op;
    this.left = left;
    this.right = right;
  }
}

function evaluate(n) {
  if (n.left === null && n.right === null) return n.val;
  const a = evaluate(n.left), b = evaluate(n.right);
  switch (n.op) {
    case "+": return a + b;
    case "-": return a - b;
    case "*": return a * b;
    default:  return a / b;
  }
}

const leaf = (v) => new Node({ val: v });
const root = new Node({
  op: "*",
  left: new Node({ op: "+", left: leaf(3), right: leaf(2) }),
  right: new Node({ op: "+", left: leaf(4), right: leaf(5) }),
});
console.log(evaluate(root)); // 45
