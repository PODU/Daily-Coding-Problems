// Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
// Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }
}

function maxPathSum(root) {
  let best = -Infinity;
  function gain(n) {
    if (!n) return 0;
    const l = Math.max(0, gain(n.left));
    const r = Math.max(0, gain(n.right));
    best = Math.max(best, n.val + l + r);
    return n.val + Math.max(l, r);
  }
  gain(root);
  return best;
}

const root = new Node(-10, new Node(9), new Node(20, new Node(15), new Node(7)));
console.log(maxPathSum(root)); // expected 42
