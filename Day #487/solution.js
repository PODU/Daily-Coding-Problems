// Day 487: Find all cousins of a target node (same level, different parent).
// BFS level by level tracking each node's parent; on the target's level collect nodes
// whose parent differs from the target's parent. Time O(n), Space O(n).
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
    const next = [];
    for (const [node, parent] of queue) {
      level.push([node, parent]);
      if (node.val === target) targetParent = parent;
      if (node.left) next.push([node.left, node]);
      if (node.right) next.push([node.right, node]);
    }
    if (targetParent !== null) {
      return level
        .filter(([n, p]) => p !== targetParent && n.val !== target)
        .map(([n]) => n.val);
    }
    queue = next;
  }
  return [];
}

const root = new Node(1,
  new Node(2, new Node(4), new Node(5)),
  new Node(3, null, new Node(6)));
for (const v of cousins(root, 4)) console.log(v); // 6
