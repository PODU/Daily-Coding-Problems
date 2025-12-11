// Count nodes in a complete binary tree.
// Compare left/right spine heights: full subtree -> 2^h-1, else recurse.
// Time: O(log^2 n), Space: O(log n).

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
  if (lh === rh) return (1 << lh) - 1;
  return 1 + countNodes(root.left) + countNodes(root.right);
}

const root = new Node(1, new Node(2, new Node(4), new Node(5)), new Node(3, new Node(6)));
console.log(countNodes(root)); // 6
