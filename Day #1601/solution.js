// Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
// Reconstruct path by tracking the chosen child. Time O(n), space O(h).
"use strict";

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

// returns { sum, path } from node down to a leaf.
function minPath(node) {
  if (node.left === null && node.right === null) {
    return { sum: node.val, path: [node.val] };
  }
  let best = null;
  for (const child of [node.left, node.right]) {
    if (child === null) continue;
    const r = minPath(child);
    if (best === null || r.sum < best.sum) best = r;
  }
  return { sum: node.val + best.sum, path: [node.val, ...best.path] };
}

function main() {
  const root = new Node(10);
  root.left = new Node(5);
  root.right = new Node(5);
  root.left.right = new Node(2);
  root.right.right = new Node(1);
  root.right.right.left = new Node(-1);

  const { sum, path } = minPath(root);
  console.log(`The minimum path is [${path.join(", ")}], which has sum ${sum}.`);
}

main();
