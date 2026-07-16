// Convert to full binary tree by removing single-child nodes (post-order recursion).
// O(N) time, O(H) space.
class Node {
  constructor(val) { this.val = val; this.l = null; this.r = null; }
}

function fullify(n) {
  if (n === null) return null;
  n.l = fullify(n.l);
  n.r = fullify(n.r);
  if (n.l === null && n.r !== null) return n.r;
  if (n.l !== null && n.r === null) return n.l;
  return n;
}

function serialize(n) {
  if (n === null) return "";
  if (n.l === null && n.r === null) return String(n.val);
  return `${n.val}(${serialize(n.l)}, ${serialize(n.r)})`;
}

const nodes = Array.from({ length: 8 }, (_, i) => new Node(i));
nodes[0].l = nodes[1]; nodes[0].r = nodes[2];
nodes[1].l = nodes[3];
nodes[2].r = nodes[4];
nodes[3].r = nodes[5];
nodes[4].l = nodes[6]; nodes[4].r = nodes[7];
const root = fullify(nodes[0]);
console.log(serialize(root)); // 0(5, 4(6, 7))
