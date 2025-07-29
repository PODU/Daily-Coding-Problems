// Day 48: Reconstruct binary tree from preorder + inorder.
// Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function buildTree(preorder, inorder) {
  const pos = new Map();
  inorder.forEach((v, i) => pos.set(v, i));
  let preIdx = 0;
  function helper(inL, inR) {
    if (inL > inR) return null;
    const rootVal = preorder[preIdx++];
    const root = new Node(rootVal);
    const mid = pos.get(rootVal);
    root.left = helper(inL, mid - 1);
    root.right = helper(mid + 1, inR);
    return root;
  }
  return helper(0, inorder.length - 1);
}

function levelOrder(root) {
  const out = [], q = [root];
  while (q.length) {
    const n = q.shift();
    out.push(n.val);
    if (n.left) q.push(n.left);
    if (n.right) q.push(n.right);
  }
  return out;
}

const pre = ["a", "b", "d", "e", "c", "f", "g"];
const ino = ["d", "b", "e", "a", "f", "c", "g"];
// Level-order traversal confirms reconstruction: a b c d e f g
console.log(levelOrder(buildTree(pre, ino)).join(" "));
