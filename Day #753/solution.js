// Day 753: Ternary Search Tree with insert and search.
// Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.
"use strict";

class TSTNode {
    constructor(c) {
        this.c = c;
        this.isEnd = false;
        this.left = this.mid = this.right = null;
    }
}

class TST {
    constructor() { this.root = null; }

    _insert(node, s, i) {
        const c = s[i];
        if (!node) node = new TSTNode(c);
        if (c < node.c)       node.left  = this._insert(node.left,  s, i);
        else if (c > node.c)  node.right = this._insert(node.right, s, i);
        else if (i + 1 < s.length)
            node.mid = this._insert(node.mid, s, i + 1);
        else node.isEnd = true;
        return node;
    }

    insert(s) { if (s) this.root = this._insert(this.root, s, 0); }

    search(s) {
        let node = this.root, i = 0;
        while (node) {
            const c = s[i];
            if (c < node.c)       node = node.left;
            else if (c > node.c)  node = node.right;
            else {
                if (i + 1 === s.length) return node.isEnd;
                node = node.mid;
                i++;
            }
        }
        return false;
    }
}

const tst = new TST();
for (const w of ["code", "cob", "be", "ax", "war", "we"]) tst.insert(w);
for (const q of ["code", "cob", "cod", "ax", "hello"])
    console.log(`${q}: ${tst.search(q)}`);
