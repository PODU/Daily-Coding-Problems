// BFS level-order: sum each level, track the level (1-indexed) with min sum.
// Time O(n), Space O(width).

class Node {
  constructor(val, l = null, r = null) { this.val = val; this.l = l; this.r = r; }
}

function minSumLevel(root) {
  if (!root) return [-1, 0];
  let q = [root];
  let level = 0, bestLevel = 1, bestSum = Infinity;
  while (q.length) {
    const next = [];
    let sum = 0;
    level++;
    for (const n of q) {
      sum += n.val;
      if (n.l) next.push(n.l);
      if (n.r) next.push(n.r);
    }
    if (sum < bestSum) { bestSum = sum; bestLevel = level; }
    q = next;
  }
  return [bestLevel, bestSum];
}

const root = new Node(10, new Node(2, new Node(4), new Node(5)), new Node(3));
const [lvl, sum] = minSumLevel(root);
console.log(`Level with minimum sum: ${lvl} (sum = ${sum})`);
