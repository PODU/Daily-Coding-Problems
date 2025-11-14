// Day 596: Invert a binary tree (mirror it).
// Approach: recursively swap left/right children. Time O(n), Space O(h).
'use strict';

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function invert(root) {
    if (!root) return;
    [root.left, root.right] = [root.right, root.left];
    invert(root.left);
    invert(root.right);
}

function main() {
    const a = new Node('a'), b = new Node('b'), c = new Node('c');
    const d = new Node('d'), e = new Node('e'), f = new Node('f');
    a.left = b; a.right = c;
    b.left = d; b.right = e;
    c.left = f;

    invert(a);

    let q = [a];
    while (q.length) {
        const next = [];
        const line = [];
        for (const n of q) {
            line.push(n.val);
            if (n.left) next.push(n.left);
            if (n.right) next.push(n.right);
        }
        console.log(line.join(' '));
        q = next;
    }
}

main();
