// Day 1755: Count nodes in a COMPLETE binary tree in better than O(n).
// Compare left/right spine heights: if equal, subtree is perfect (2^h - 1);
// else 1 + recurse on both children. Time O(log^2 n), Space O(log n).
'use strict';

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function leftHeight(n) {
  let h = 0;
  while (n) { h++; n = n.left; }
  return h;
}
function rightHeight(n) {
  let h = 0;
  while (n) { h++; n = n.right; }
  return h;
}
function countNodes(root) {
  if (!root) return 0;
  const lh = leftHeight(root), rh = rightHeight(root);
  if (lh === rh) return (1 << lh) - 1; // perfect subtree
  return 1 + countNodes(root.left) + countNodes(root.right);
}

function main() {
  // Complete binary tree with 6 nodes (values 1..6):
  //          1
  //        /   \
  //       2     3
  //      / \   /
  //     4   5 6
  const root = new Node(1);
  root.left = new Node(2);
  root.right = new Node(3);
  root.left.left = new Node(4);
  root.left.right = new Node(5);
  root.right.left = new Node(6);

  console.log(countNodes(root)); // expected: 6
}

main();
