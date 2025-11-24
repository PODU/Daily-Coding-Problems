// Validate BST with inclusive bounds: left<=node, right>=node (duplicates allowed both sides).
// Recursive (low,high) bound check. Time O(n), Space O(h). Node.js.

class TreeNode {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function isValid(node, low, high) {
  if (node === null) return true;
  if (node.val < low || node.val > high) return false;
  return isValid(node.left, low, node.val) &&
    isValid(node.right, node.val, high);
}

function isBST(root) {
  return isValid(root, -Infinity, Infinity);
}

function main() {
  // Tree A (valid): root=5, left=3(l=2,r=5), right=8(l=8,r=9)
  const a = new TreeNode(5);
  a.left = new TreeNode(3);
  a.right = new TreeNode(8);
  a.left.left = new TreeNode(2);
  a.left.right = new TreeNode(5);
  a.right.left = new TreeNode(8);
  a.right.right = new TreeNode(9);

  // Tree B (invalid): root=5, left=3, right=4
  const b = new TreeNode(5);
  b.left = new TreeNode(3);
  b.right = new TreeNode(4);

  console.log(isBST(a) ? "true" : "false");
  console.log(isBST(b) ? "true" : "false");
}

main();
