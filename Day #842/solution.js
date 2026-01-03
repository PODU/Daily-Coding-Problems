// Day 842: invert (mirror) a binary tree by swapping children at every node.
// Recursive DFS. O(n) time, O(h) stack space.
class Node {
  constructor(v, l = null, r = null) { this.v = v; this.l = l; this.r = r; }
}

function invert(root) {
  if (root === null) return null;
  [root.l, root.r] = [root.r, root.l];
  invert(root.l);
  invert(root.r);
  return root;
}

function levelOrder(root) {
  if (root === null) return "";
  const out = [], q = [root];
  while (q.length) {
    const n = q.shift();
    out.push(n.v);
    if (n.l) q.push(n.l);
    if (n.r) q.push(n.r);
  }
  return out.join(" ");
}

const a = new Node('a'), b = new Node('b'), c = new Node('c'),
      d = new Node('d'), e = new Node('e'), f = new Node('f');
a.l = b; a.r = c; b.l = d; b.r = e; c.l = f;
invert(a);
console.log(levelOrder(a)); // a c b f e d
