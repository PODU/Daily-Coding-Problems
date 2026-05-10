// Day 1501: Most frequent subtree sum.
// Approach: post-order DFS, accumulate subtree sums in a Map, pick max count.
// Time: O(n), Space: O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function mostFrequentSubtreeSum(root) {
  const freq = new Map();
  function dfs(node) {
    if (!node) return 0;
    const sum = node.val + dfs(node.left) + dfs(node.right);
    freq.set(sum, (freq.get(sum) || 0) + 1);
    return sum;
  }
  dfs(root);
  let best = 0;
  for (const c of freq.values()) best = Math.max(best, c);
  const res = [];
  for (const [k, v] of freq) if (v === best) res.push(k);
  return res.sort((a, b) => a - b);
}

const root = new Node(5, new Node(2), new Node(-5));
console.log(mostFrequentSubtreeSum(root).join(" "));
