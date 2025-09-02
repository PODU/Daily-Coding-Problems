// Day 204: Count nodes of a complete binary tree faster than O(n).
// Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
// Time: O(log^2 n), Space: O(log n).
class Node { constructor() { this.left = null; this.right = null; } }

function leftHeight(n) { let h = 0; while (n) { h++; n = n.left; } return h; }
function rightHeight(n) { let h = 0; while (n) { h++; n = n.right; } return h; }

function countNodes(root) {
  if (!root) return 0;
  const lh = leftHeight(root), rh = rightHeight(root);
  if (lh === rh) return (1 << lh) - 1; // perfect subtree
  return 1 + countNodes(root.left) + countNodes(root.right);
}

const n = [null];
for (let i = 1; i < 7; i++) n.push(new Node());
n[1].left = n[2]; n[1].right = n[3];
n[2].left = n[4]; n[2].right = n[5];
n[3].left = n[6];
console.log(countNodes(n[1])); // 6
