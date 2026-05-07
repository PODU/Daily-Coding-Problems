// Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
// For each root i, combine all left BSTs of (lo..i-1) with all right BSTs of
// (i+1..hi). Count is Catalan(N). Time/Space O(Catalan(N) * N).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function build(lo, hi) {
  if (lo > hi) return [null];
  const trees = [];
  for (let i = lo; i <= hi; ++i)
    for (const l of build(lo, i - 1))
      for (const r of build(i + 1, hi)) trees.push(new Node(i, l, r));
  return trees;
}

function preorder(n) {
  if (!n) return [];
  return [n.val, ...preorder(n.left), ...preorder(n.right)];
}

const trees = build(1, 3);
console.log(trees.length); // 5
for (const t of trees) console.log(preorder(t));
