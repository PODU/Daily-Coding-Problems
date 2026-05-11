// Reconstruct BST from postorder. Process postorder from the right with an
// upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).

class Node {
  constructor(v) {
    this.val = v;
    this.left = null;
    this.right = null;
  }
}

function bstFromPostorder(post) {
  let idx = post.length - 1;
  function build(bound) {
    if (idx < 0 || post[idx] < bound) return null;
    const root = new Node(post[idx]);
    idx--;
    root.right = build(root.val);
    root.left = build(bound);
    return root;
  }
  return build(-Infinity);
}

function preorder(root, out) {
  if (!root) return;
  out.push(root.val);
  preorder(root.left, out);
  preorder(root.right, out);
}

const post = [2, 4, 3, 8, 7, 5];
const root = bstFromPostorder(post);
const pre = [];
preorder(root, pre);
console.log(pre.join(" "));
