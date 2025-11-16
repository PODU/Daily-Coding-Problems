// Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
// Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).
'use strict';

class Node {
    constructor() {
        this.sum = 0;
        this.ch = new Map();
    }
}

class PrefixMapSum {
    constructor() {
        this.root = new Node();
        this.values = new Map();
    }
    insert(key, value) {
        const delta = value - (this.values.get(key) || 0);
        this.values.set(key, value);
        let node = this.root;
        for (const c of key) {
            if (!node.ch.has(c)) node.ch.set(c, new Node());
            node = node.ch.get(c);
            node.sum += delta;
        }
    }
    sum(prefix) {
        let node = this.root;
        for (const c of prefix) {
            if (!node.ch.has(c)) return 0;
            node = node.ch.get(c);
        }
        return node.sum;
    }
}

function main() {
    const m = new PrefixMapSum();
    m.insert("columnar", 3);
    console.log(m.sum("col")); // 3
    m.insert("column", 2);
    console.log(m.sum("col")); // 5
}

main();
