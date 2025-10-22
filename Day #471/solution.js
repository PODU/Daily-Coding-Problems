// Generate all structurally distinct BSTs with values 1..N via recursive root selection.
// Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.
"use strict";

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function generate(start, end) {
    if (start > end) return [null];
    const trees = [];
    for (let i = start; i <= end; i++) {
        const lefts = generate(start, i - 1);
        const rights = generate(i + 1, end);
        for (const l of lefts) {
            for (const r of rights) {
                const root = new Node(i);
                root.left = l;
                root.right = r;
                trees.push(root);
            }
        }
    }
    return trees;
}

function preorder(root, out) {
    if (root === null) return;
    out.push(root.val);
    preorder(root.left, out);
    preorder(root.right, out);
}

function main() {
    const N = 3;
    const trees = generate(1, N);
    console.log("Number of BSTs: " + trees.length);
    for (const t of trees) {
        const out = [];
        preorder(t, out);
        console.log(out.join(" "));
    }
}

main();
