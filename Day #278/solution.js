// Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
// Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).
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
    for (const L of build(lo, r - 1)) {
      for (const R of build(r + 1, hi)) {
        const n = new Node(r);
        n.left = L;
        n.right = R;
        res.push(n);
      }
    }
  }
  return res;
}

function preorder(n) {
  if (n === null) return "#";
  return `${n.val} ${preorder(n.left)} ${preorder(n.right)}`;
}

const N = 3;
const trees = build(1, N);
console.log("Count:", trees.length); // 5
for (const t of trees) console.log(preorder(t));
