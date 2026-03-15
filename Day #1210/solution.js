// Day 1210: Floor and ceiling of a target in a BST.
// Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).
class Node {
  constructor(val) { this.val = val; this.l = null; this.r = null; }
}

function insert(root, v) {
  if (root === null) return new Node(v);
  if (v < root.val) root.l = insert(root.l, v);
  else root.r = insert(root.r, v);
  return root;
}

function floorCeil(root, x) {
  let fl = null, ce = null;
  while (root) {
    if (root.val === x) return [x, x];
    if (root.val < x) { fl = root.val; root = root.r; }
    else { ce = root.val; root = root.l; }
  }
  return [fl, ce];
}

let root = null;
for (const v of [8, 4, 12, 2, 6, 10, 14]) root = insert(root, v);
const [fl, ce] = floorCeil(root, 7);
console.log(`floor=${fl === null ? "None" : fl} ceiling=${ce === null ? "None" : ce}`); // floor=6 ceiling=8
