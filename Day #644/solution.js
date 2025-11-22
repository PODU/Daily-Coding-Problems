// Day 644: Count unival subtrees (all nodes share one value).
// Approach: post-order DFS; a node is unival iff both children are unival and
// their values match the node's. Count as we recurse.
// Time: O(n), Space: O(h).
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
    const left = dfs(node.left);
    const right = dfs(node.right);
    if (!left || !right) return false;
    if (node.left && node.left.val !== node.val) return false;
    if (node.right && node.right.val !== node.val) return false;
    count++;
    return true;
  }
  dfs(root);
  return count;
}

const root = new Node(0,
  new Node(1),
  new Node(0,
    new Node(1, new Node(1), new Node(1)),
    new Node(0)));
console.log(countUnival(root)); // 5
