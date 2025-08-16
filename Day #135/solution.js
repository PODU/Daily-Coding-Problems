// Day 135: Minimum root-to-leaf path sum (with path reconstruction).
// DFS over the tree. O(n) time, O(h) recursion space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function minPath(r) {
  if (!r) return [Infinity, []];
  if (!r.left && !r.right) return [r.val, [r.val]];
  let best = [Infinity, []];
  for (const c of [r.left, r.right]) {
    if (!c) continue;
    const sub = minPath(c);
    if (sub[0] < best[0]) best = sub;
  }
  return [best[0] + r.val, [r.val, ...best[1]]];
}

const root = new Node(10, new Node(5, null, new Node(2)), new Node(5, null, new Node(1, new Node(-1))));
const [total, path] = minPath(root);
console.log(`${total} (path [${path.join(", ")}])`);
