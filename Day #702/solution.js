// Day 702: Serialize/deserialize a binary tree.
// Approach: preorder traversal with '#' null markers, comma-separated tokens.
// Both directions O(n) time and space.
class Node {
  constructor(val, left = null, right = null) { this.val = val; this.left = left; this.right = right; }
}

function serialize(root) {
  const out = [];
  const go = (n) => {
    if (n === null) { out.push("#"); return; }
    out.push(String(n.val));
    go(n.left);
    go(n.right);
  };
  go(root);
  return out.join(",");
}

function deserialize(s) {
  const toks = s.split(",");
  let i = 0;
  const go = () => {
    const t = toks[i++];
    if (t === "#") return null;
    const n = new Node(t);
    n.left = go();
    n.right = go();
    return n;
  };
  return go();
}

const node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
console.log(deserialize(serialize(node)).left.left.val); // left.left
