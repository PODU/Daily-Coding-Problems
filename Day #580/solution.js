// Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minPath(root) {
  let best = { sum: Infinity, path: [] };
  function dfs(node, cur) {
    if (node === null) return;
    cur.push(node.val);
    if (node.left === null && node.right === null) {
      const s = cur.reduce((a, b) => a + b, 0);
      if (s < best.sum) {
        best.sum = s;
        best.path = cur.slice();
      }
    } else {
      dfs(node.left, cur);
      dfs(node.right, cur);
    }
    cur.pop();
  }
  dfs(root, []);
  return best;
}

const root = new Node(10);
root.left = new Node(5);
root.left.right = new Node(2);
root.right = new Node(5);
root.right.right = new Node(1);
root.right.right.left = new Node(-1);

const best = minPath(root);
console.log("[" + best.path.join(", ") + "], which has sum " + best.sum + ".");
