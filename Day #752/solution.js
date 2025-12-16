// Day 752: Level-order (BFS) traversal of a binary tree.
// Time: O(n), Space: O(w) where w is the max width of the tree.
"use strict";

class Node {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function levelOrder(root) {
    const out = [];
    if (!root) return out;
    const q = [root];
    let head = 0;
    while (head < q.length) {
        const n = q[head++];
        out.push(n.val);
        if (n.left) q.push(n.left);
        if (n.right) q.push(n.right);
    }
    return out;
}

const root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
console.log(levelOrder(root).join(", "));
