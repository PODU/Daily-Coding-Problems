// Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
// Time: O(n), Space: O(h) recursion.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function countUnival(root) {
  let count = 0;
  // returns true if subtree is unival; increments count.
  function dfs(n) {
    if (n === null) return true;
    const l = dfs(n.left);
    const r = dfs(n.right);
    if (!l || !r) return false;
    if (n.left && n.left.val !== n.val) return false;
    if (n.right && n.right.val !== n.val) return false;
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
console.log(countUnival(root));
