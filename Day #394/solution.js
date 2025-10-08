// Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function hasPathSum(root, k) {
  if (root === null) return false;
  if (root.left === null && root.right === null) return k - root.val === 0;
  return hasPathSum(root.left, k - root.val) || hasPathSum(root.right, k - root.val);
}

const root = new Node(8, new Node(4, new Node(2), new Node(6)), new Node(13, null, new Node(19)));
console.log(hasPathSum(root, 18));
