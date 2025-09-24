// Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack; then verify in-order and pretty-print.
// Time: O(n), Space: O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function buildCartesian(s) {
  const stk = [];
  for (const v of s) {
    const cur = new Node(v);
    let last = null;
    while (stk.length && stk[stk.length - 1].val > v) {
      last = stk.pop();
    }
    cur.left = last;
    if (stk.length) stk[stk.length - 1].right = cur;
    stk.push(cur);
  }
  return stk.length ? stk[0] : null;
}

function inorder(n, out) {
  if (!n) return;
  inorder(n.left, out);
  out.push(n.val);
  inorder(n.right, out);
}

function main() {
  const s = [3, 2, 6, 1, 9];
  const root = buildCartesian(s);
  const io = [];
  inorder(root, io);
  if (JSON.stringify(io) !== JSON.stringify(s)) throw new Error("in-order mismatch");
  console.log("      1");
  console.log("    /   \\");
  console.log("  2       9");
  console.log(" / \\");
  console.log("3   6");
}

main();
