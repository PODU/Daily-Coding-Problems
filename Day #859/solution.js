// Day 859: Bottom view of a binary tree.
// Approach: BFS by horizontal distance; last node seen at each hd wins (lowest).
// Time: O(n log n) for ordering, Space: O(n).
'use strict';

class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function bottomView(root) {
  const hdMap = new Map();
  const q = [[root, 0]];
  let i = 0;
  while (i < q.length) {
    const [node, hd] = q[i++];
    hdMap.set(hd, node.val);
    if (node.left) q.push([node.left, hd - 1]);
    if (node.right) q.push([node.right, hd + 1]);
  }
  return [...hdMap.keys()].sort((a, b) => a - b).map((hd) => hdMap.get(hd));
}

const root = new Node(5,
  new Node(3, new Node(1, new Node(0)), new Node(4)),
  new Node(7, new Node(6), new Node(9, new Node(8))));
console.log('[' + bottomView(root).join(', ') + ']');
