// Day 83: Invert (mirror) a binary tree by swapping children recursively.
// Time O(n), Space O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function invert(root) {
  if (!root) return null;
  [root.left, root.right] = [root.right, root.left];
  invert(root.left);
  invert(root.right);
  return root;
}

function levelOrder(root) {
  const out = [], q = root ? [root] : [];
  while (q.length) {
    const n = q.shift();
    out.push(n.val);
    if (n.left) q.push(n.left);
    if (n.right) q.push(n.right);
  }
  return out.join(" ");
}

const a = new Node("a",
  new Node("b", new Node("d"), new Node("e")),
  new Node("c", new Node("f")));
console.log("before:", levelOrder(a)); // a b c d e f
invert(a);
console.log("after: ", levelOrder(a)); // a c b f e d
