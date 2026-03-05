// Level-order (BFS) traversal of a binary tree using a queue. O(n) time, O(n) space.
"use strict";

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function levelOrder(root) {
    const res = [];
    if (!root) return res;
    const queue = [root];
    let head = 0;
    while (head < queue.length) {
        const cur = queue[head++];
        res.push(cur.val);
        if (cur.left) queue.push(cur.left);
        if (cur.right) queue.push(cur.right);
    }
    return res;
}

function main() {
    const root = new Node(1);
    root.left = new Node(2);
    root.right = new Node(3);
    root.right.left = new Node(4);
    root.right.right = new Node(5);

    const vals = levelOrder(root);
    console.log(vals.join(", "));
}

main();
