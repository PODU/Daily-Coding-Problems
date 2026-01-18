// Complete-tree node count: if left height == right height subtree is perfect (2^h-1), else recurse. O(log^2 n).
'use strict';

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function leftHeight(n) { let h = 0; while (n) { h++; n = n.left; } return h; }
function rightHeight(n) { let h = 0; while (n) { h++; n = n.right; } return h; }

function countNodes(root) {
  if (!root) return 0;
  const lh = leftHeight(root), rh = rightHeight(root);
  if (lh === rh) return (1 << lh) - 1; // perfect subtree
  return 1 + countNodes(root.left) + countNodes(root.right);
}

function main() {
  // Complete tree with 6 nodes (values 1..6 in level order):
  //         1
  //       /   \
  //      2     3
  //     / \   /
  //    4   5 6
  const n = {};
  for (let v = 1; v <= 6; v++) n[v] = new Node(v);
  n[1].left = n[2]; n[1].right = n[3];
  n[2].left = n[4]; n[2].right = n[5];
  n[3].left = n[6];

  console.log(countNodes(n[1])); // 6
}

main();
