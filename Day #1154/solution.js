// Day 1154: Minimum root-to-leaf path sum.
// DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minPath(root) {
  let best = { sum: Infinity, path: [] };
  function dfs(n, path, total) {
    if (!n) return;
    path.push(n.val);
    total += n.val;
    if (!n.left && !n.right) {
      if (total < best.sum) best = { sum: total, path: path.slice() };
    } else {
      dfs(n.left, path, total);
      dfs(n.right, path, total);
    }
    path.pop();
  }
  dfs(root, [], 0);
  return best;
}

const root = new Node(10, new Node(5, null, new Node(2)), new Node(5, null, new Node(1, new Node(-1))));
const { sum, path } = minPath(root);
console.log(`[${path.join(", ")}], which has sum ${sum}`); // [10, 5, 1, -1], which has sum 15
