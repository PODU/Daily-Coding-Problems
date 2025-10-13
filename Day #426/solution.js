// Day 426: Level of binary tree with minimum sum (levels 0-indexed; root = level 0).
// BFS level-order summing each level, track minimum. Time O(n), Space O(width).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minLevel(root) {
  if (!root) return [-1, 0];
  let q = [root], level = 0, bestLevel = 0, best = Infinity;
  while (q.length) {
    let s = 0, next = [];
    for (const n of q) {
      s += n.val;
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    if (s < best) {
      best = s;
      bestLevel = level;
    }
    q = next;
    level++;
  }
  return [bestLevel, best];
}

const root = new Node(1, new Node(2), new Node(3));
const [lvl, s] = minLevel(root);
console.log(`Level with minimum sum: ${lvl} (sum = ${s})`);
