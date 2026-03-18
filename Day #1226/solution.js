// Cousins in a binary tree: BFS level by level tracking parent; collect same-level
// nodes with a different parent than target. Time O(n), Space O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function cousins(root, target) {
  if (!root) return [];
  let queue = [[root, null]]; // [node, parent]
  while (queue.length) {
    const level = [];
    let targetParent = null;
    let found = false;
    const next = [];
    for (const [node, parent] of queue) {
      level.push([node, parent]);
      if (node.val === target) {
        found = true;
        targetParent = parent;
      }
      if (node.left) next.push([node.left, node]);
      if (node.right) next.push([node.right, node]);
    }
    if (found) {
      return level
        .filter(([n, p]) => n.val !== target && p !== targetParent)
        .map(([n]) => n.val);
    }
    queue = next;
  }
  return [];
}

const root = new Node(1,
  new Node(2, new Node(4), new Node(5)),
  new Node(3, null, new Node(6)));

console.log("[" + cousins(root, 4).join(", ") + "]");
