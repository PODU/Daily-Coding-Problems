// Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(n), Space O(n).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function boustrophedon(root) {
  const out = [];
  if (!root) return out;
  let queue = [root];
  let leftToRight = true;
  while (queue.length) {
    const next = [];
    const level = [];
    for (const n of queue) {
      level.push(n.val);
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    out.push(...(leftToRight ? level : level.reverse()));
    leftToRight = !leftToRight;
    queue = next;
  }
  return out;
}

const root = new Node(1,
  new Node(2, new Node(4), new Node(5)),
  new Node(3, new Node(6), new Node(7)));
console.log('[' + boustrophedon(root).join(', ') + ']');
