// Root-to-leaf paths via DFS, carrying current path; record at leaves. Time O(n), Space O(h).
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
    if (!node.left && !node.right) {
      res.push([...path]);
    } else {
      dfs(node.left, path);
      dfs(node.right, path);
    }
    path.pop();
  }
  dfs(root, []);
  return res;
}

const root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
const res = rootToLeafPaths(root);
console.log("[" + res.map(p => "[" + p.join(", ") + "]").join(", ") + "]");
