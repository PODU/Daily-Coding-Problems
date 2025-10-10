// Largest BST subtree: bottom-up DFS returning [isBST, size, min, max]; combine children.
// Time O(n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function largestBST(root) {
  let best = 0;
  function dfs(node) {
    if (!node) return [true, 0, Infinity, -Infinity];
    const [lB, lS, lMin, lMax] = dfs(node.left);
    const [rB, rS, rMin, rMax] = dfs(node.right);
    if (lB && rB && lMax < node.val && node.val < rMin) {
      const size = lS + rS + 1;
      best = Math.max(best, size);
      return [true, size, Math.min(node.val, lMin), Math.max(node.val, rMax)];
    }
    return [false, 0, 0, 0];
  }
  dfs(root);
  return best;
}

const root = new Node(10,
  new Node(5, new Node(1), new Node(8)),
  new Node(15, null, new Node(7)));
console.log(largestBST(root));
