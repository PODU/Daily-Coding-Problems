// Serialize/deserialize a binary tree via preorder with null markers.
// Time O(n), Space O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val; this.left = left; this.right = right;
  }
}

function serialize(root) {
  const out = [];
  const go = (n) => {
    if (n === null) { out.push('#'); return; }
    out.push(String(n.val).replace(/\\/g, '\\\\').replace(/\|/g, '\\|'));
    go(n.left); go(n.right);
  };
  go(root);
  return out.join('|');
}

function deserialize(s) {
  const toks = [];
  let cur = '';
  for (let i = 0; i < s.length; i++) {
    if (s[i] === '\\') { cur += s[++i]; }
    else if (s[i] === '|') { toks.push(cur); cur = ''; }
    else cur += s[i];
  }
  toks.push(cur);
  let pos = 0;
  const go = () => {
    const v = toks[pos++];
    if (v === '#') return null;
    return new Node(v, go(), go());
  };
  return go();
}

const node = new Node('root', new Node('left', new Node('left.left')), new Node('right'));
console.log(deserialize(serialize(node)).left.left.val);
