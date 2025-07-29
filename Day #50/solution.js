// Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
// Time: O(n), Space: O(h).
class Node {
  constructor({ val = null, op = null, left = null, right = null }) {
    this.val = val; this.op = op; this.left = left; this.right = right;
  }
}

const OPS = {
  "+": (a, b) => a + b,
  "-": (a, b) => a - b,
  "*": (a, b) => a * b,
  "/": (a, b) => a / b,
};

function evaluate(n) {
  if (n.op === null) return n.val;
  return OPS[n.op](evaluate(n.left), evaluate(n.right));
}

const root = new Node({
  op: "*",
  left: new Node({ op: "+", left: new Node({ val: 3 }), right: new Node({ val: 2 }) }),
  right: new Node({ op: "+", left: new Node({ val: 4 }), right: new Node({ val: 5 }) }),
});
console.log(evaluate(root));
