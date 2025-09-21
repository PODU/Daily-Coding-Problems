// Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
class Node { constructor(v) { this.v = v; this.l = null; this.r = null; } }

function insert(root, v) {
  if (!root) return new Node(v);
  if (v < root.v) root.l = insert(root.l, v); else root.r = insert(root.r, v);
  return root;
}

function floorCeil(root, x) {
  let floor = null, ceil = null;
  while (root) {
    if (root.v === x) return [root.v, root.v];
    if (root.v < x) { floor = root.v; root = root.r; }
    else { ceil = root.v; root = root.l; }
  }
  return [floor, ceil];
}

let root = null;
for (const v of [8, 4, 12, 2, 6, 10, 14]) root = insert(root, v);
const [fl, ce] = floorCeil(root, 5);
console.log(`Floor: ${fl === null ? "None" : fl}, Ceiling: ${ce === null ? "None" : ce}`); // Floor: 4, Ceiling: 6
