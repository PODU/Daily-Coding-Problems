// Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
// Time: O(n) for both, Space: O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function serialize(root) {
  const out = [];
  (function rec(n) {
    if (n === null) { out.push("#"); return; }
    out.push(String(n.val)); // assumes values contain no ','
    rec(n.left);
    rec(n.right);
  })(root);
  return out.join(",");
}

function deserialize(s) {
  const toks = s.split(",");
  let i = 0;
  function rec() {
    const t = toks[i++];
    if (t === "#") return null;
    return new Node(t, rec(), rec());
  }
  return rec();
}

const node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
console.log(deserialize(serialize(node)).left.left.val);
