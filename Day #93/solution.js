// Day 93: Largest BST subtree size. Post-order DFS returning [isBst, size, min,
// max] per node and combining children. O(n) time, O(h) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function largestBST(root) {
  let best = 0;
  function dfs(n) {
    if (!n) return [true, 0, Infinity, -Infinity]; // bst, size, min, max
    const [lb, ls, lmin, lmax] = dfs(n.left);
    const [rb, rs, rmin, rmax] = dfs(n.right);
    if (lb && rb && lmax < n.val && n.val < rmin) {
      const size = ls + rs + 1;
      best = Math.max(best, size);
      return [true, size, Math.min(lmin, n.val), Math.max(rmax, n.val)];
    }
    return [false, 0, 0, 0];
  }
  dfs(root);
  return best;
}

const root = new Node(10,
  new Node(5, new Node(1), new Node(8)),
  new Node(15, null, new Node(7)));
console.log(largestBST(root)); // 3
