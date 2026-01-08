// Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function largestBST(root) {
  let best = 0;
  function dfs(n) {
    if (n === null) return { isBST: true, size: 0, mn: Infinity, mx: -Infinity };
    const l = dfs(n.left), r = dfs(n.right);
    if (l.isBST && r.isBST && l.mx < n.val && n.val < r.mn) {
      const sz = l.size + r.size + 1;
      best = Math.max(best, sz);
      return { isBST: true, size: sz, mn: Math.min(n.val, l.mn), mx: Math.max(n.val, r.mx) };
    }
    return { isBST: false, size: 0, mn: 0, mx: 0 };
  }
  dfs(root);
  return best;
}

const root = new Node(10, new Node(5, new Node(1), new Node(8)), new Node(15, null, new Node(7)));
console.log(largestBST(root));
