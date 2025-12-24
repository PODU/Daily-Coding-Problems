// Prune binary tree to a full binary tree: post-order recursion; if a node has
// exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function prune(root) {
  if (root === null) return null;
  root.left = prune(root.left);
  root.right = prune(root.right);
  if (root.left && !root.right) return root.left;
  if (root.right && !root.left) return root.right;
  return root;
}

function main() {
  const n = Array.from({ length: 8 }, (_, i) => new Node(i));
  n[0].left = n[1]; n[0].right = n[2];
  n[1].left = n[3];
  n[2].right = n[4];
  n[3].right = n[5];
  n[4].left = n[6]; n[4].right = n[7];

  const root = prune(n[0]);

  let q = [root];
  const lines = [];
  while (q.length) {
    const level = [];
    const next = [];
    for (const cur of q) {
      level.push(cur.val);
      if (cur.left) next.push(cur.left);
      if (cur.right) next.push(cur.right);
    }
    lines.push(level.join(" "));
    q = next;
  }
  console.log(lines.join("\n"));
}

main();
