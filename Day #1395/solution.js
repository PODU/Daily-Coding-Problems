// LRU cache via JS Map (preserves insertion order); get/set O(1), evict oldest on overflow. O(1) time, O(n) space.

class LRUCache {
    constructor(capacity) {
        this.cap = capacity;
        this.map = new Map();
    }

    get(key) {
        if (!this.map.has(key)) return null;
        const val = this.map.get(key);
        this.map.delete(key);
        this.map.set(key, val); // mark most recently used
        return val;
    }

    set(key, value) {
        if (this.map.has(key)) this.map.delete(key);
        this.map.set(key, value);
        if (this.map.size > this.cap) {
            const lru = this.map.keys().next().value; // oldest key
            this.map.delete(lru);
        }
    }
}

function fmt(v) {
    return v === null ? "null" : String(v);
}

function main() {
    const cache = new LRUCache(2);
    cache.set(1, 1);
    cache.set(2, 2);
    console.log(fmt(cache.get(1)));   // 1
    cache.set(3, 3);                  // evicts key 2
    console.log(fmt(cache.get(2)));   // null
    cache.set(4, 4);                  // evicts key 1
    console.log(fmt(cache.get(1)));   // null
    console.log(fmt(cache.get(3)));   // 3
    console.log(fmt(cache.get(4)));   // 4
}

main();
