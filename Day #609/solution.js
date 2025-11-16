// Day 609: Inorder successor in a BST using parent pointers.
// Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).
'use strict';

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
        this.parent = null;
    }
}

function inorderSuccessor(node) {
    if (!node) return null;
    if (node.right) {
        let c = node.right;
        while (c.left) c = c.left;
        return c;
    }
    let cur = node, p = node.parent;
    while (p && cur === p.right) { cur = p; p = p.parent; }
    return p;
}

function attach(parent, child) {
    if (child) child.parent = parent;
    return child;
}

function main() {
    const n10 = new Node(10), n5 = new Node(5), n30 = new Node(30);
    const n22 = new Node(22), n35 = new Node(35);
    n10.left = attach(n10, n5);
    n10.right = attach(n10, n30);
    n30.left = attach(n30, n22);
    n30.right = attach(n30, n35);

    const s = inorderSuccessor(n22);
    console.log(s ? s.val : "null"); // 30
}

main();
