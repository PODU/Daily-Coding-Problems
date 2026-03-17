// Post-order DFS computing subtree sums, count frequencies in a hashmap,
// return sum(s) with max frequency. O(n) time, O(n) space.
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
    const s = node.val + dfs(node.left) + dfs(node.right);
    freq.set(s, (freq.get(s) || 0) + 1);
    return s;
  }
  dfs(root);
  let best = 0;
  for (const c of freq.values()) best = Math.max(best, c);
  const res = [];
  for (const [s, c] of freq) if (c === best) res.push(s);
  res.sort((a, b) => a - b);
  return res;
}

const root = new Node(5, new Node(2), new Node(-5));
console.log(mostFrequentSubtreeSum(root).join(" "));
