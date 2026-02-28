// Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function bottomView(root) {
  const hdMap = new Map();
  const q = [[root, 0]];
  while (q.length) {
    const [node, hd] = q.shift();
    hdMap.set(hd, node.val);
    if (node.left) q.push([node.left, hd - 1]);
    if (node.right) q.push([node.right, hd + 1]);
  }
  const keys = [...hdMap.keys()].sort((a, b) => a - b);
  return keys.map((k) => hdMap.get(k));
}

const root = new Node(5,
  new Node(3, new Node(1, new Node(0)), new Node(4)),
  new Node(7, new Node(6), new Node(9, new Node(8))));
console.log("[" + bottomView(root).join(", ") + "]");
