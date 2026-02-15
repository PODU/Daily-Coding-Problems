// K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
class Node {
    constructor(val, children = []) { this.val = val; this.children = children; }
}

function isMirror(L, R) {
    if (!L && !R) return true;
    if (!L || !R || L.val !== R.val) return false;
    const n = L.children.length;
    if (R.children.length !== n) return false;
    for (let i = 0; i < n; i++)
        if (!isMirror(L.children[i], R.children[n - 1 - i])) return false;
    return true;
}

function isSymmetric(root) {
    if (!root) return true;
    const n = root.children.length;
    for (let i = 0; i < Math.floor(n / 2); i++)
        if (!isMirror(root.children[i], root.children[n - 1 - i])) return false;
    return true;
}

// Symmetric: 4 -> [3,5,3], first 3 -> [9], last 3 -> [9]
const root = new Node(4, [new Node(3, [new Node(9)]), new Node(5), new Node(3, [new Node(9)])]);
console.log("Symmetric: " + isSymmetric(root));

// Asymmetric: 1 -> [2,3]
const r2 = new Node(1, [new Node(2), new Node(3)]);
console.log("Symmetric: " + isSymmetric(r2));
