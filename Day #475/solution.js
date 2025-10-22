// Tree locking with parent pointers + per-node lockedDescendantCount.
// lock/unlock are O(h): walk ancestors once to check, once to update counts.
"use strict";

class Node {
    constructor(name) {
        this.name = name;
        this.parent = null;
        this.left = null;
        this.right = null;
        this.locked = false;
        this.lockedDescendantCount = 0;
    }

    isLocked() { return this.locked; }

    anyAncestorLocked() {
        for (let p = this.parent; p; p = p.parent)
            if (p.locked) return true;
        return false;
    }

    lock() {
        if (this.locked) return false;
        if (this.lockedDescendantCount > 0) return false; // a descendant is locked
        if (this.anyAncestorLocked()) return false;       // an ancestor is locked
        this.locked = true;
        for (let p = this.parent; p; p = p.parent) p.lockedDescendantCount++;
        return true;
    }

    unlock() {
        if (!this.locked) return false;
        this.locked = false;
        for (let p = this.parent; p; p = p.parent) p.lockedDescendantCount--;
        return true;
    }
}

function child(p, c, left) {
    if (left) p.left = c; else p.right = c;
    c.parent = p;
    return c;
}

function cap(b) { return b ? "True" : "False"; }

function main() {
    const n1 = new Node("node1");
    const n2 = new Node("node2");
    const n3 = new Node("node3");
    const n4 = new Node("node4");
    const n5 = new Node("node5");
    child(n1, n2, true); child(n1, n3, false);
    child(n2, n4, true); child(n2, n5, false);

    console.log("lock(node4): " + cap(n4.lock()));      // True
    console.log("lock(node2): " + cap(n2.lock()));      // False (descendant locked)
    console.log("unlock(node4): " + cap(n4.unlock()));  // True
    console.log("lock(node2): " + cap(n2.lock()));      // True
    console.log("lock(node1): " + cap(n1.lock()));      // False (descendant locked)
    console.log("lock(node5): " + cap(n5.lock()));      // False (ancestor locked)
    console.log("unlock(node2): " + cap(n2.unlock()));  // True
    console.log("lock(node1): " + cap(n1.lock()));      // True
}

main();
