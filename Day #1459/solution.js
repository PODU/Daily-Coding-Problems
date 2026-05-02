// Invert binary tree by swapping children recursively; print level-order (BFS).
// Time O(n), Space O(n).
'use strict';

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function invert(root) {
  if (!root) return;
  const tmp = root.left;
  root.left = root.right;
  root.right = tmp;
  invert(root.left);
  invert(root.right);
}

function main() {
  const a = new Node('a'), b = new Node('b'), c = new Node('c');
  const d = new Node('d'), e = new Node('e'), f = new Node('f');
  a.left = b; a.right = c;
  b.left = d; b.right = e;
  c.left = f;

  invert(a);

  const out = [];
  const q = [a];
  while (q.length) {
    const n = q.shift();
    if (!n) continue;
    out.push(n.val);
    q.push(n.left);
    q.push(n.right);
  }
  console.log(out.join(' '));
}

main();
