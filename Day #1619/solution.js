// Subtree check via serialization with sentinels + substring search.
// Time: O(n+m), Space: O(n+m).
'use strict';

class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function serialize(node) {
  if (node === null) return ',#';
  return ',' + node.val + serialize(node.left) + serialize(node.right);
}

function isSubtree(s, t) {
  return serialize(s).includes(serialize(t));
}

function main() {
  const s = new TreeNode(3);
  s.left = new TreeNode(4);
  s.right = new TreeNode(5);
  s.left.left = new TreeNode(1);
  s.left.right = new TreeNode(2);

  const t = new TreeNode(4);
  t.left = new TreeNode(1);
  t.right = new TreeNode(2);

  console.log(isSubtree(s, t) ? 'true' : 'false');
}

main();
