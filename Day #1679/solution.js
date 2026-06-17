// Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
// per subtree, tracking the global best. Time O(n), Space O(h).
class TreeNode {
  constructor(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }
}

function largestBST(root) {
  let best = 0;
  function dfs(node) {
    if (!node) return [true, 0, Infinity, -Infinity];
    const [lBst, lSz, lMn, lMx] = dfs(node.left);
    const [rBst, rSz, rMn, rMx] = dfs(node.right);
    if (lBst && rBst && lMx < node.val && node.val < rMn) {
      const sz = lSz + rSz + 1;
      best = Math.max(best, sz);
      return [true, sz, Math.min(node.val, lMn), Math.max(node.val, rMx)];
    }
    return [false, 0, -Infinity, Infinity];
  }
  dfs(root);
  return best;
}

const root = new TreeNode(10,
  new TreeNode(5, new TreeNode(1), new TreeNode(8)),
  new TreeNode(15, null, new TreeNode(7)));
console.log(largestBST(root)); // 3
