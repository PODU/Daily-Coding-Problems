// Day 112: LCA with parent pointers - equalize depths, walk up together. O(h).
class Node {
  constructor(val, parent = null) {
    this.val = val;
    this.parent = parent;
    this.l = null;
    this.r = null;
  }
}

const depth = (n) => {
  let d = 0;
  while (n) { n = n.parent; d++; }
  return d;
};

function lca(a, b) {
  let da = depth(a), db = depth(b);
  while (da > db) { a = a.parent; da--; }
  while (db > da) { b = b.parent; db--; }
  while (a !== b) { a = a.parent; b = b.parent; }
  return a;
}

const root = new Node(3);
root.l = new Node(5, root); root.r = new Node(1, root);
root.l.l = new Node(6, root.l); root.l.r = new Node(2, root.l);
root.r.l = new Node(0, root.r); root.r.r = new Node(8, root.r);
root.l.r.l = new Node(7, root.l.r); root.l.r.r = new Node(4, root.l.r);
console.log(lca(root.l, root.r).val);          // 3
console.log(lca(root.l.r.l, root.l.r.r).val);  // 2
