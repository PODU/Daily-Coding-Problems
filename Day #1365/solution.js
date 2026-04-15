// Count complete-tree nodes: if left height == right height subtree is perfect (2^h-1),
// else recurse. Time O(log^2 n), Space O(log n) recursion.
'use strict';

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
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

function main() {
    const root = new Node(1);
    root.left = new Node(2);
    root.right = new Node(3);
    root.left.left = new Node(4);
    root.left.right = new Node(5);
    root.right.left = new Node(6);
    console.log(countNodes(root));
}

main();
