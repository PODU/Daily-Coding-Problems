// Day 1478: Return a deepest node of a binary tree.
// Single DFS returning [depth, node]; keep the deeper subtree's result.
// Time O(N), Space O(H).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function deepest(root) {
  function dfs(node) {
    if (!node) return [0, null];
    const [ld, ln] = dfs(node.left);
    const [rd, rn] = dfs(node.right);
    if (ld >= rd) return [ld + 1, node.left ? ln : node];
    return [rd + 1, rn];
  }
  return dfs(root)[1];
}

const root = new Node('a', new Node('b', new Node('d')), new Node('c'));
console.log(deepest(root).val); // d
