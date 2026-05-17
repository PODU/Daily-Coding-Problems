// Invert (mirror) a binary tree by swapping left/right children of every node.
// Time O(n), Space O(h) recursion.
class Node {
  constructor(val, l = null, r = null) {
    this.val = val;
    this.l = l;
    this.r = r;
  }
}

function invert(root) {
  if (root === null) return null;
  [root.l, root.r] = [root.r, root.l];
  invert(root.l);
  invert(root.r);
  return root;
}

function preorder(root, out) {
  if (root === null) return;
  out.push(root.val);
  preorder(root.l, out);
  preorder(root.r, out);
}

const a = new Node("a", new Node("b", new Node("d"), new Node("e")), new Node("c", new Node("f")));
const before = [];
preorder(a, before);
invert(a);
const after = [];
preorder(a, after);
console.log("before (preorder):", before.join(" "));
console.log("after  (preorder):", after.join(" "));
