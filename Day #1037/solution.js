// Sorted array -> height-balanced BST: recursively pick middle as root.
// Time: O(n), Space: O(log n) recursion.

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function build(a, lo, hi) {
    if (lo > hi) return null;
    const mid = lo + ((hi - lo) >> 1);
    const root = new Node(a[mid]);
    root.left = build(a, lo, mid - 1);
    root.right = build(a, mid + 1, hi);
    return root;
}

function printRotated(node, depth) {
    if (!node) return;
    printRotated(node.right, depth + 1);
    console.log(" ".repeat(depth * 4) + node.val);
    printRotated(node.left, depth + 1);
}

function inorder(node, out) {
    if (!node) return;
    inorder(node.left, out);
    out.push(node.val);
    inorder(node.right, out);
}

function main() {
    const a = [-10, -3, 0, 5, 9];
    const root = build(a, 0, a.length - 1);
    console.log("Height-balanced BST (rotated 90 deg):");
    printRotated(root, 0);
    const io = [];
    inorder(root, io);
    console.log("In-order: " + io.join(" "));
}

main();
