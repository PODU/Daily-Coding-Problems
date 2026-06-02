// Approach: Symmetric k-ary tree via isMirror recursion comparing children mirror-wise.
// Time O(n), Space O(h) recursion.
'use strict';

class Node {
    constructor(val) { this.val = val; this.children = []; }
}

function isMirror(a, b) {
    if (a === null && b === null) return true;
    if (a === null || b === null) return false;
    if (a.val !== b.val) return false;
    if (a.children.length !== b.children.length) return false;
    const k = a.children.length;
    for (let i = 0; i < k; i++)
        if (!isMirror(a.children[i], b.children[k - 1 - i])) return false;
    return true;
}

function isSymmetric(root) {
    if (root === null) return true;
    return isMirror(root, root);
}

const root = new Node(4);
const c1 = new Node(3); c1.children.push(new Node(9));
const c2 = new Node(5);
const c3 = new Node(3); c3.children.push(new Node(9));
root.children = [c1, c2, c3];
console.log(isSymmetric(root) ? "true" : "false");
