// Day 1036: Reconstruct BST from postorder traversal.
// Approach: walk postorder in reverse (root,right,left) using value bounds.
// Time: O(n), Space: O(h) recursion.
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

function printSideways(n, depth = 0) {
  if (!n) return;
  printSideways(n.right, depth + 1);
  console.log(" ".repeat(depth * 4) + n.val);
  printSideways(n.left, depth + 1);
}

const post = [2, 4, 3, 8, 7, 5];
const root = reconstruct(post);
console.log("Reconstructed BST (rotated 90 deg, root=5):");
printSideways(root);
