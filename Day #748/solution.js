// Most frequent subtree sum: post-order DFS computes each subtree sum, count in a Map.
// Time: O(n), Space: O(n).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function mostFrequentSubtreeSum(root) {
  const count = new Map();
  function dfs(n) {
    if (!n) return 0;
    const s = n.val + dfs(n.left) + dfs(n.right);
    count.set(s, (count.get(s) || 0) + 1);
    return s;
  }
  dfs(root);
  let best = 0, bestCount = -1;
  for (const [sum, c] of count) if (c > bestCount) { bestCount = c; best = sum; }
  return best;
}

const root = new Node(5, new Node(2), new Node(-5));
console.log(mostFrequentSubtreeSum(root)); // 2
