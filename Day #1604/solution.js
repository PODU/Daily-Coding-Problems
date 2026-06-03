// BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
// Floor: go right recording when val<=x else left. Ceiling: symmetric.
class Node {
  constructor(val) { this.val = val; this.l = null; this.r = null; }
}

function insert(root, v) {
  if (root === null) return new Node(v);
  if (v < root.val) root.l = insert(root.l, v);
  else root.r = insert(root.r, v);
  return root;
}

function floorBST(root, x) {
  let res = null;
  while (root) {
    if (root.val === x) return x;
    if (root.val < x) { res = root.val; root = root.r; }
    else root = root.l;
  }
  return res;
}

function ceilBST(root, x) {
  let res = null;
  while (root) {
    if (root.val === x) return x;
    if (root.val > x) { res = root.val; root = root.l; }
    else root = root.r;
  }
  return res;
}

const show = (v) => (v === null ? "None" : String(v));

function query(root, x) {
  console.log(`x=${x} -> floor ${show(floorBST(root, x))}, ceiling ${show(ceilBST(root, x))}`);
}

let root = null;
for (const v of [8, 4, 12, 2, 6, 10, 14]) root = insert(root, v);
query(root, 5);   // floor 4, ceiling 6
query(root, 8);   // floor 8, ceiling 8
query(root, 15);  // floor 14, ceiling None
query(root, 1);   // floor None, ceiling 2
