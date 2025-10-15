// Day 435: Reconstruct a binary tree from preorder + inorder traversals.
// Approach: recurse, using a Map of inorder value->index to find roots in O(1).
// Time: O(n), Space: O(n).
"use strict";

function reconstruct(preorder, inorder) {
  const idx = new Map();
  inorder.forEach((v, i) => idx.set(v, i));
  let pi = 0;

  function build(inL, inR) {
    if (inL > inR) return null;
    const rootVal = preorder[pi++];
    const root = { val: rootVal, left: null, right: null };
    const mid = idx.get(rootVal);
    root.left = build(inL, mid - 1);
    root.right = build(mid + 1, inR);
    return root;
  }

  return build(0, inorder.length - 1);
}

const preorder = ["a", "b", "d", "e", "c", "f", "g"];
const inorder = ["d", "b", "e", "a", "f", "c", "g"];
const root = reconstruct(preorder, inorder);

// Print level-order: a b c d e f g
const out = [];
const q = [root];
while (q.length) {
  const n = q.shift();
  out.push(n.val);
  if (n.left) q.push(n.left);
  if (n.right) q.push(n.right);
}
console.log(out.join(" "));
