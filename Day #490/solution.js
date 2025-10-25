// Day 490: Bottom view of a binary tree.
// BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
// order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function bottomView(root) {
  if (!root) return [];
  const hdToVal = new Map();
  let queue = [[root, 0]];
  while (queue.length) {
    const next = [];
    for (const [node, hd] of queue) {
      hdToVal.set(hd, node.val); // last in BFS order = lowest
      if (node.left) next.push([node.left, hd - 1]);
      if (node.right) next.push([node.right, hd + 1]);
    }
    queue = next;
  }
  return [...hdToVal.keys()].sort((a, b) => a - b).map((hd) => hdToVal.get(hd));
}

const root = new Node(5,
  new Node(3, new Node(1, new Node(0)), new Node(4)),
  new Node(7, new Node(6), new Node(9, new Node(8))));
console.log(bottomView(root)); // [0, 1, 3, 6, 8, 9]
