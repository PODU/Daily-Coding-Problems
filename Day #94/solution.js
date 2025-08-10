// Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
// node consider a path bending through it. O(n) time, O(h) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function maxPathSum(root) {
  let best = -Infinity;
  function gain(n) {
    if (!n) return 0;
    const l = Math.max(gain(n.left), 0);
    const r = Math.max(gain(n.right), 0);
    best = Math.max(best, n.val + l + r);
    return n.val + Math.max(l, r);
  }
  gain(root);
  return best;
}

const root = new Node(-10, new Node(9),
  new Node(20, new Node(15), new Node(7)));
console.log(maxPathSum(root)); // 42
