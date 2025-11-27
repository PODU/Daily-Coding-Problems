// Day 664: Maximum path sum between any two nodes in a binary tree.
// Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).
class Node { constructor(val, l = null, r = null) { this.val = val; this.l = l; this.r = r; } }

function maxPathSum(root) {
  let best = -Infinity;
  function gain(n) {
    if (!n) return 0;
    const lg = Math.max(0, gain(n.l));
    const rg = Math.max(0, gain(n.r));
    best = Math.max(best, n.val + lg + rg);
    return n.val + Math.max(lg, rg);
  }
  gain(root);
  return best;
}

const root = new Node(-10, new Node(9), new Node(20, new Node(15), new Node(7)));
console.log(maxPathSum(root)); // 42
