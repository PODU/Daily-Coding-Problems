// Height-balanced binary tree check.
// Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function check(root) {
    if (root === null) return 0;
    const l = check(root.left);
    if (l === -1) return -1;
    const r = check(root.right);
    if (r === -1) return -1;
    if (Math.abs(l - r) > 1) return -1;
    return Math.max(l, r) + 1;
}

function isBalanced(root) {
    return check(root) !== -1;
}

function main() {
    const a = new Node(1);
    a.left = new Node(2);
    a.right = new Node(3);
    a.left.left = new Node(4);

    const b = new Node(1);
    b.left = new Node(2);
    b.left.left = new Node(3);

    console.log("Balanced tree: " + (isBalanced(a) ? "true" : "false"));
    console.log("Unbalanced tree: " + (isBalanced(b) ? "true" : "false"));
}

main();
