// DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function deepestNode(root) {
  let maxDepth = -1, deepest = null;
  function dfs(node, depth) {
    if (!node) return;
    if (depth > maxDepth) { maxDepth = depth; deepest = node.val; }
    dfs(node.left, depth + 1);
    dfs(node.right, depth + 1);
  }
  dfs(root, 0);
  return deepest;
}

const a = new Node("a", new Node("b", new Node("d")), new Node("c"));
console.log(deepestNode(a));
