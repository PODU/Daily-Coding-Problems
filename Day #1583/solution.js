// Day 1583: Bottom view of a binary tree.
// BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
"use strict";

class Node {
  constructor(val, l = null, r = null) { this.val = val; this.l = l; this.r = r; }
}

function bottomView(root) {
  if (!root) return [];
  const hdVal = new Map();
  const q = [[root, 0]];
  let head = 0;
  while (head < q.length) {
    const [n, hd] = q[head++];
    hdVal.set(hd, n.val); // overwrite -> deepest wins
    if (n.l) q.push([n.l, hd - 1]);
    if (n.r) q.push([n.r, hd + 1]);
  }
  return [...hdVal.keys()].sort((a, b) => a - b).map((k) => hdVal.get(k));
}

const root = new Node(5,
  new Node(3, new Node(1, new Node(0)), new Node(4)),
  new Node(7, new Node(6), new Node(9, new Node(8))));
console.log(bottomView(root)); // [0, 1, 3, 6, 8, 9]
