// Day 997: Serialize / deserialize a binary tree.
// Preorder traversal with "#" markers for null children, comma-joined.
// Both serialize and deserialize are O(n) time and space.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function serialize(root) {
  const out = [];
  (function go(n) {
    if (n === null) {
      out.push("#");
      return;
    }
    out.push(String(n.val));
    go(n.left);
    go(n.right);
  })(root);
  return out.join(",");
}

function deserialize(s) {
  const toks = s.split(",");
  let i = 0;
  function go() {
    const v = toks[i++];
    if (v === "#") return null;
    const n = new Node(v);
    n.left = go();
    n.right = go();
    return n;
  }
  return go();
}

const node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
const s = serialize(node);
console.log(s);
console.assert(deserialize(serialize(node)).left.left.val === "left.left");
console.log("assertion passed:", deserialize(s).left.left.val);
