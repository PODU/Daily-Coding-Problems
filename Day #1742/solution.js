// Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
// Time/Space O(Catalan(N)*N).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function build(lo, hi) {
  if (lo > hi) return [null];
  const res = [];
  for (let r = lo; r <= hi; r++) {
    const lefts = build(lo, r - 1);
    const rights = build(r + 1, hi);
    for (const l of lefts)
      for (const ri of rights) {
        const root = new Node(r);
        root.left = l;
        root.right = ri;
        res.push(root);
      }
  }
  return res;
}

function preorder(node, out) {
  if (node === null) return;
  out.push(node.val);
  preorder(node.left, out);
  preorder(node.right, out);
}

const N = 3;
const trees = build(1, N);
console.log(trees.length);
for (const t of trees) {
  const out = [];
  preorder(t, out);
  console.log(out.join(","));
}
