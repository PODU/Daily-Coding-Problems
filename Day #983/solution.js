// Root-to-leaf paths via DFS, appending to the current path and recording it at each leaf.
// Time O(n) nodes (O(n*h) including path copies), Space O(h) recursion.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function rootToLeafPaths(root) {
  const res = [];
  function dfs(node, path) {
    if (!node) return;
    path.push(node.val);
    if (!node.left && !node.right) res.push([...path]);
    else { dfs(node.left, path); dfs(node.right, path); }
    path.pop();
  }
  dfs(root, []);
  return res;
}

const root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
console.log(JSON.stringify(rootToLeafPaths(root))); // [[1,2],[1,3,4],[1,3,5]]
