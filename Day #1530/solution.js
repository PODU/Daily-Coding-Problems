// Zigzag (boustrophedon) level-order traversal: alternate direction each level.
// BFS level by level, reverse odd levels. O(n) time, O(n) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function zigzag(root) {
  const res = [];
  if (!root) return res;
  let q = [root];
  let leftToRight = true;
  while (q.length) {
    const next = [];
    const level = [];
    for (const n of q) {
      level.push(n.val);
      if (n.left) next.push(n.left);
      if (n.right) next.push(n.right);
    }
    if (!leftToRight) level.reverse();
    res.push(...level);
    leftToRight = !leftToRight;
    q = next;
  }
  return res;
}

const root = new Node(1,
  new Node(2, new Node(4), new Node(5)),
  new Node(3, new Node(6), new Node(7)));

console.log(JSON.stringify(zigzag(root))); // [1,3,2,4,5,6,7]
