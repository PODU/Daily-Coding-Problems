// Level of a binary tree with the minimum node-value sum.
// BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minSumLevel(root) {
  if (!root) return -1;
  let queue = [root];
  let level = 0;
  let bestLevel = 0;
  let bestSum = Infinity;
  while (queue.length) {
    let sum = 0;
    const next = [];
    for (const n of queue) {
      sum += n.val;
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    if (sum < bestSum) {
      bestSum = sum;
      bestLevel = level;
    }
    queue = next;
    level++;
  }
  return bestLevel;
}

const root = new Node(5, new Node(2, new Node(-5)), new Node(3));
console.log("Level with minimum sum:", minSumLevel(root));
