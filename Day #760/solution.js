// Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
// forward Fisher-Yates that swaps node values in place using O(1) extra space
// at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
// A deterministic LCG is used so the demo output is reproducible.
"use strict";

class Node {
    constructor(val, next = null) { this.val = val; this.next = next; }
}

class LCG {
    constructor(seed) { this.s = BigInt(seed); }
    next() {
        // BigInt: s * 1103515245 overflows the 2^53 float mantissa otherwise.
        this.s = (this.s * 1103515245n + 12345n) & 0x7fffffffn;
        return Number(this.s);
    }
}

function shuffle(head, rng) {
    for (let p = head; p; p = p.next) {
        let m = 0;
        for (let t = p; t; t = t.next) m++;
        let r = rng.next() % m;
        let q = p;
        while (r-- > 0) q = q.next;
        const tmp = p.val; p.val = q.val; q.val = tmp;
    }
}

function printList(head) {
    const parts = [];
    for (let p = head; p; p = p.next) parts.push(p.val);
    console.log(parts.join(" -> "));
}

let head = new Node(1);
let cur = head;
for (let v = 2; v <= 5; v++) { cur.next = new Node(v); cur = cur.next; }

process.stdout.write("original: "); printList(head);
shuffle(head, new LCG(42));
process.stdout.write("shuffled: "); printList(head);
