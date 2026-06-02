// Reconstruct binary tree from preorder+inorder using inorder index hashmap
// and a moving preorder pointer. Time O(n), Space O(n).
"use strict";

class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function build(preorder, inorder) {
  const idx = new Map();
  inorder.forEach((v, i) => idx.set(v, i));
  let preIdx = 0;

  function helper(inL, inR) {
    if (inL > inR) return null;
    const rootVal = preorder[preIdx++];
    const root = new Node(rootVal);
    const mid = idx.get(rootVal);
    root.left = helper(inL, mid - 1);
    root.right = helper(mid + 1, inR);
    return root;
  }
  return helper(0, inorder.length - 1);
}

function preorderT(n, out) { if (n) { out.push(n.val); preorderT(n.left, out); preorderT(n.right, out); } }
function inorderT(n, out) { if (n) { inorderT(n.left, out); out.push(n.val); inorderT(n.right, out); } }
function postorderT(n, out) { if (n) { postorderT(n.left, out); postorderT(n.right, out); out.push(n.val); } }

const pre = ['a', 'b', 'd', 'e', 'c', 'f', 'g'];
const ino = ['d', 'b', 'e', 'a', 'f', 'c', 'g'];
const root = build(pre, ino);

const po = [], pr = [], io = [];
postorderT(root, po);
preorderT(root, pr);
inorderT(root, io);
console.log("postorder: " + po.join(" "));
console.log("preorder:  " + pr.join(" "));
console.log("inorder:   " + io.join(" "));
