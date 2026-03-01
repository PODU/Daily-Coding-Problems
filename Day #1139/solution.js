// LCA via parent pointers: equalize depths then walk up together. O(h) time, O(1) space.
class Node {
  constructor(val) {
    this.val = val;
    this.parent = null;
    this.left = null;
    this.right = null;
  }
}

function depth(n) {
  let d = 0;
  while (n.parent) { n = n.parent; d++; }
  return d;
}

function lca(a, b) {
  let da = depth(a), db = depth(b);
  while (da > db) { a = a.parent; da--; }
  while (db > da) { b = b.parent; db--; }
  while (a !== b) { a = a.parent; b = b.parent; }
  return a;
}

function link(p, c, left) {
  if (left) p.left = c; else p.right = c;
  c.parent = p;
  return c;
}

const n1 = new Node(1);
const n2 = new Node(2), n3 = new Node(3);
const n4 = new Node(4), n5 = new Node(5);
const n6 = new Node(6), n7 = new Node(7);
link(n1, n2, true); link(n1, n3, false);
link(n2, n4, true); link(n2, n5, false);
link(n3, n6, true); link(n3, n7, false);

console.log(lca(n4, n5).val);
console.log(lca(n4, n6).val);
