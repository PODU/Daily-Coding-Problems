// Day 994: Print binary tree nodes level by level (BFS).
// Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function levelOrder(root) {
  const out = [];
  const q = root ? [root] : [];
  let i = 0;
  while (i < q.length) {
    const n = q[i++];
    out.push(n.val);
    if (n.left) q.push(n.left);
    if (n.right) q.push(n.right);
  }
  return out;
}

const root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
console.log(levelOrder(root).join(", ")); // 1, 2, 3, 4, 5
