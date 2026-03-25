// Day 1263: All root-to-leaf paths in a binary tree.
// DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function rootToLeaf(root) {
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
console.log(JSON.stringify(rootToLeaf(root)));
