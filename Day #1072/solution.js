// BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function minSumLevel(root) {
  if (!root) return [-1, 0];
  const q = [root];
  let level = 1, minLevel = 1, minSum = Infinity;
  while (q.length) {
    const sz = q.length;
    let sum = 0;
    for (let i = 0; i < sz; i++) {
      const cur = q.shift();
      sum += cur.val;
      if (cur.left)  q.push(cur.left);
      if (cur.right) q.push(cur.right);
    }
    if (sum < minSum) { minSum = sum; minLevel = level; }
    level++;
  }
  return [minLevel, minSum];
}

// Tree 1
const r1 = new Node(1, new Node(2, new Node(4), new Node(5)), new Node(3, null, new Node(6)));
let [lvl, sm] = minSumLevel(r1);
console.log(`Level with min sum: ${lvl} (sum=${sm})`);

// Tree 2
const r2 = new Node(10, new Node(2), new Node(3));
[lvl, sm] = minSumLevel(r2);
console.log(`Level with min sum: ${lvl} (sum=${sm})`);
