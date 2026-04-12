// Day 1349: Reconstruct a BST from its postorder traversal.
// Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function reconstruct(post) {
  let idx = post.length - 1;
  function build(bound) {
    if (idx < 0 || post[idx] < bound) return null;
    const node = new Node(post[idx--]);
    node.right = build(node.val);
    node.left = build(bound);
    return node;
  }
  return build(-Infinity);
}

function preorder(n, out) { if (!n) return; out.push(n.val); preorder(n.left, out); preorder(n.right, out); }
function inorder(n, out) { if (!n) return; inorder(n.left, out); out.push(n.val); inorder(n.right, out); }

const root = reconstruct([2, 4, 3, 8, 7, 5]);
const pre = [], ino = [];
preorder(root, pre); inorder(root, ino);
console.log("Root:", root.val);
console.log("Preorder:", pre.join(" "));
console.log("Inorder:", ino.join(" "));
