// Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing the output order on alternate levels.
// Time: O(n), Space: O(n).
"use strict";

class TreeNode {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function boustrophedon(root) {
    const out = [];
    if (!root) return out;
    let queue = [root];
    let leftToRight = true;
    while (queue.length) {
        const sz = queue.length;
        const level = new Array(sz);
        const next = [];
        for (let i = 0; i < sz; i++) {
            const node = queue[i];
            const idx = leftToRight ? i : sz - 1 - i;
            level[idx] = node.val;
            if (node.left) next.push(node.left);
            if (node.right) next.push(node.right);
        }
        out.push(...level);
        queue = next;
        leftToRight = !leftToRight;
    }
    return out;
}

function main() {
    const root = new TreeNode(1,
        new TreeNode(2, new TreeNode(4), new TreeNode(5)),
        new TreeNode(3, new TreeNode(6), new TreeNode(7)));
    console.log("[" + boustrophedon(root).join(", ") + "]"); // [1, 3, 2, 4, 5, 6, 7]
}

main();
