// Day 1112 - Minimum root-to-leaf path sum (return the path)
// Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minPath(root) {
  let best = { sum: Infinity, path: [] };
  function dfs(node, path, s) {
    if (!node) return;
    path.push(node.val);
    s += node.val;
    if (!node.left && !node.right) {
      if (s < best.sum) { best.sum = s; best.path = path.slice(); }
    } else {
      dfs(node.left, path, s);
      dfs(node.right, path, s);
    }
    path.pop();
  }
  dfs(root, [], 0);
  return best;
}

const root = new Node(10,
  new Node(5, null, new Node(2)),
  new Node(5, null, new Node(1, new Node(-1))));
const { path, sum } = minPath(root);
console.log(`[${path.join(", ")}], which has sum ${sum}`);
