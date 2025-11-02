// Reconstruct BST from postorder: scan right-to-left (preorder of root,right,left)
// with an upper-bound recursion. Time O(n), space O(n).
'use strict';

class Node { constructor(v){ this.val = v; this.left = null; this.right = null; } }

function reconstruct(post) {
  let idx = post.length - 1;
  function build(bound) {
    if (idx < 0 || post[idx] < bound) return null;
    const root = new Node(post[idx--]);
    root.right = build(root.val);
    root.left = build(bound);
    return root;
  }
  return build(-Infinity);
}

function preorder(n, out){ if(!n) return; out.push(n.val); preorder(n.left,out); preorder(n.right,out); }
function inorder(n, out){ if(!n) return; inorder(n.left,out); out.push(n.val); inorder(n.right,out); }

const post = [2, 4, 3, 8, 7, 5];
const root = reconstruct(post);
const pre = [], ino = [];
preorder(root, pre); inorder(root, ino);
console.log("preorder: " + pre.join(" "));
console.log("inorder:  " + ino.join(" "));
