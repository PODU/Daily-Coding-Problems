// Day 724: Evaluate an arithmetic expression binary tree.
// Approach: Post-order recursion; leaves are ints, internal nodes are operators.
// Time: O(n), Space: O(h).

class Node {
  constructor({ val = null, op = null, left = null, right = null } = {}) {
    this.val = val; this.op = op; this.left = left; this.right = right;
  }
}

function evaluate(root) {
  if (root.left === null && root.right === null) return root.val;
  const l = evaluate(root.left), r = evaluate(root.right);
  switch (root.op) {
    case '+': return l + r;
    case '-': return l - r;
    case '*': return l * r;
    case '/': return l / r;
  }
}

const tree = new Node({ op: '*',
  left: new Node({ op: '+', left: new Node({ val: 3 }), right: new Node({ val: 2 }) }),
  right: new Node({ op: '+', left: new Node({ val: 4 }), right: new Node({ val: 5 }) }) });
console.log(evaluate(tree)); // 45
