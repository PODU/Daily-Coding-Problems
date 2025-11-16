// Day 611: All O(1) structure (plus / minus / get_max / get_min).
// Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map. All ops O(1).
'use strict';

class Bucket {
    constructor(value) {
        this.value = value;
        this.keys = new Set();
        this.prev = null;
        this.next = null;
    }
}

class AllOne {
    constructor() {
        this.head = new Bucket(0);
        this.tail = new Bucket(0);
        this.head.next = this.tail;
        this.tail.prev = this.head;
        this.map = new Map();
    }
    _insertAfter(node, value) {
        const b = new Bucket(value);
        b.prev = node; b.next = node.next;
        node.next.prev = b; node.next = b;
        return b;
    }
    _remove(node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }
    plus(key) {
        if (this.map.has(key)) {
            const cur = this.map.get(key);
            let nxt = cur.next;
            if (nxt === this.tail || nxt.value !== cur.value + 1) nxt = this._insertAfter(cur, cur.value + 1);
            nxt.keys.add(key);
            this.map.set(key, nxt);
            cur.keys.delete(key);
            if (cur.keys.size === 0) this._remove(cur);
        } else {
            let first = this.head.next;
            if (first === this.tail || first.value !== 1) first = this._insertAfter(this.head, 1);
            first.keys.add(key);
            this.map.set(key, first);
        }
    }
    minus(key) {
        if (!this.map.has(key)) return;
        const cur = this.map.get(key);
        if (cur.value === 1) {
            this.map.delete(key);
        } else {
            let prv = cur.prev;
            if (prv === this.head || prv.value !== cur.value - 1) prv = this._insertAfter(cur.prev, cur.value - 1);
            prv.keys.add(key);
            this.map.set(key, prv);
        }
        cur.keys.delete(key);
        if (cur.keys.size === 0) this._remove(cur);
    }
    getMax() { return this.tail.prev === this.head ? "" : [...this.tail.prev.keys].reduce((a, b) => (a < b ? a : b)); }
    getMin() { return this.head.next === this.tail ? "" : [...this.head.next.keys].reduce((a, b) => (a < b ? a : b)); }
}

function main() {
    const a = new AllOne();
    a.plus("a"); a.plus("b"); a.plus("a"); // a=2, b=1
    console.log(a.getMax()); // a
    console.log(a.getMin()); // b
}

main();
