// Day 954: count unival subtrees (all nodes in subtree share one value).
// Post-order DFS, returning whether the subtree is unival. Time O(n), Space O(h).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function countUnival(root) {
  let count = 0;
  function dfs(node) {
    if (node === null) return true;
    const l = dfs(node.left);
    const r = dfs(node.right);
    if (!l || !r) return false;
    if (node.left && node.left.val !== node.val) return false;
    if (node.right && node.right.val !== node.val) return false;
    count++;
    return true;
  }
  dfs(root);
  return count;
}

const root = new Node(0, new Node(1),
  new Node(0, new Node(1, new Node(1), new Node(1)), new Node(0)));
console.log(countUnival(root)); // 5
