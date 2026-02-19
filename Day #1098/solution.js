// Day 1098: Floor and ceiling of x in a BST.
// Walk down the tree updating candidates using BST ordering.
// Time: O(h). Space: O(1).
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
  let fl = null, ce = null, cur = root;
  while (cur) {
    if (cur.val === x) return [x, x];
    if (cur.val < x) { fl = cur.val; cur = cur.r; }
    else { ce = cur.val; cur = cur.l; }
  }
  return [fl, ce];
}

let root = null;
for (const v of [8, 4, 12, 2, 6, 10, 14]) root = insert(root, v);
const [f, c] = floorCeil(root, 5);
console.log(`floor=${f === null ? "None" : f} ceil=${c === null ? "None" : c}`); // floor=4 ceil=6
