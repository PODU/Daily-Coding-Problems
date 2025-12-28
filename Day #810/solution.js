// Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(N), Space O(N).

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function boustrophedon(root) {
  const out = [];
  if (!root) return out;
  let queue = [root];
  let ltr = true;
  while (queue.length) {
    const next = [];
    const level = [];
    for (const n of queue) {
      level.push(n.val);
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    out.push(...(ltr ? level : level.reverse()));
    ltr = !ltr;
    queue = next;
  }
  return out;
}

const root = new Node(1);
root.left = new Node(2); root.right = new Node(3);
root.left.left = new Node(4); root.left.right = new Node(5);
root.right.left = new Node(6); root.right.right = new Node(7);
console.log(boustrophedon(root)); // [ 1, 3, 2, 4, 5, 6, 7 ]
