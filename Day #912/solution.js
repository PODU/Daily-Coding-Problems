// Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
'use strict';

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
    const level = queue;
    queue = [];
    let targetParent = null;
    for (const [node, par] of level) {
      if (node.val === target) targetParent = par;
      if (node.left) queue.push([node.left, node]);
      if (node.right) queue.push([node.right, node]);
    }
    if (targetParent !== null) {
      return level
        .filter(([node, par]) => par !== targetParent && node.val !== target)
        .map(([node]) => node.val);
    }
  }
  return [];
}

function main() {
  const root = new Node(1);
  root.left = new Node(2);
  root.right = new Node(3);
  root.left.left = new Node(4);
  root.left.right = new Node(5);
  root.right.right = new Node(6);

  console.log('Cousins of 4: [' + cousins(root, 4).join(', ') + ']'); // [6]
  console.log('Cousins of 6: [' + cousins(root, 6).join(', ') + ']'); // [4, 5]
}

main();
