// Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minSumLevel(root) {
  if (!root) return -1;
  let q = [root];
  let level = 0, best = 0, bestSum = Infinity;
  while (q.length) {
    let sum = 0;
    const next = [];
    for (const n of q) {
      sum += n.val;
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    if (sum < bestSum) { bestSum = sum; best = level; }
    q = next;
    level++;
  }
  return best;
}

const root = new Node(10, new Node(2, new Node(-5), new Node(1)), new Node(3));
console.log(minSumLevel(root)); // 2
