// Generate all distinct BSTs with values 1..N: recursively pick each value as
// root, combine all left/right subtree shapes. Count is Catalan(N).
// Time/Space O(Catalan(N) * N).

class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function build(lo, hi) {
  if (lo > hi) return [null];
  const res = [];
  for (let root = lo; root <= hi; root++) {
    const lefts = build(lo, root - 1);
    const rights = build(root + 1, hi);
    for (const l of lefts) {
      for (const r of rights) {
        const node = new TreeNode(root);
        node.left = l;
        node.right = r;
        res.push(node);
      }
    }
  }
  return res;
}

function preorder(node) {
  if (node === null) return "#";
  return `${node.val} ${preorder(node.left)} ${preorder(node.right)}`;
}

function main() {
  const N = 3;
  const trees = build(1, N);
  console.log(trees.length); // 5 for N=3
  for (const t of trees) {
    console.log(preorder(t));
  }
}

main();
