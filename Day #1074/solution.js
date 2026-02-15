// LCA with parent pointers: get depths via parent walk, level-up deeper node, advance both until equal. O(h) time O(1) space.
class Node {
  constructor(val) { this.val = val; this.left = this.right = this.parent = null; }
}

function link(par, child, isLeft) {
  child.parent = par;
  if (isLeft) par.left = child; else par.right = child;
  return child;
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

const n3 = new Node(3);
const n5 = link(n3, new Node(5), true);
const n1 = link(n3, new Node(1), false);
const n6 = link(n5, new Node(6), true);
const n2 = link(n5, new Node(2), false);
link(n1, new Node(0), true);
link(n1, new Node(8), false);
const n7 = link(n2, new Node(7), true);
const n4 = link(n2, new Node(4), false);

console.log(`LCA(7,4) = ${lca(n7, n4).val}`);
console.log(`LCA(6,4) = ${lca(n6, n4).val}`);
console.log(`LCA(7,1) = ${lca(n7, n1).val}`);
