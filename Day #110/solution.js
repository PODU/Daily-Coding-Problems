// Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function rootToLeaf(root) {
  const res = [];
  const dfs = (n, path) => {
    if (!n) return;
    path.push(n.val);
    if (!n.left && !n.right) res.push([...path]);
    else {
      dfs(n.left, path);
      dfs(n.right, path);
    }
    path.pop();
  };
  dfs(root, []);
  return res;
}

const root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
console.log(JSON.stringify(rootToLeaf(root)).replace(/,/g, ", "));
