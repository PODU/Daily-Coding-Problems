// Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
// same depth with different parent. Time O(n), Space O(n).
class Node {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function cousins(root, target) {
    let level = [[root, null]]; // [node, parent]
    while (level.length) {
        let targetParent = null, found = false;
        const next = [];
        for (const [node, par] of level) {
            if (node.val === target) { targetParent = par; found = true; }
            if (node.left) next.push([node.left, node]);
            if (node.right) next.push([node.right, node]);
        }
        if (found) {
            return level.filter(([n, par]) => par !== targetParent).map(([n]) => n.val);
        }
        level = next;
    }
    return [];
}

function main() {
    const root = new Node(1,
        new Node(2, new Node(4), new Node(5)),
        new Node(3, null, new Node(6)));
    console.log(cousins(root, 4));
}

main();
