// Day 80: Return a deepest node of a binary tree via DFS tracking depth.
// Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function deepestNode(root) {
  let best = -1, res = null;
  function dfs(node, depth) {
    if (!node) return;
    if (depth > best) { best = depth; res = node.val; }
    dfs(node.left, depth + 1);
    dfs(node.right, depth + 1);
  }
  dfs(root, 0);
  return res;
}

const a = new Node("a", new Node("b", new Node("d")), new Node("c"));
console.log(deepestNode(a)); // d
