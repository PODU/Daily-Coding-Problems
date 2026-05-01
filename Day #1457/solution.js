// LFU cache: Map key->node, Map freq->doubly-linked-list, track minFreq. O(1) per op.
// Time: O(1) get/set. Space: O(capacity).

class Node {
  constructor(key = 0, val = 0) {
    this.key = key;
    this.val = val;
    this.freq = 1;
    this.prev = null;
    this.next = null;
  }
}

class DLL {
  constructor() {
    this.head = new Node();
    this.tail = new Node();
    this.head.next = this.tail;
    this.tail.prev = this.head;
    this.size = 0;
  }
  addFront(n) {
    n.prev = this.head;
    n.next = this.head.next;
    this.head.next.prev = n;
    this.head.next = n;
    this.size++;
  }
  remove(n) {
    n.prev.next = n.next;
    n.next.prev = n.prev;
    this.size--;
  }
  removeLast() {
    if (this.size === 0) return null;
    const n = this.tail.prev;
    this.remove(n);
    return n;
  }
}

class LFUCache {
  constructor(capacity) {
    this.cap = capacity;
    this.minFreq = 0;
    this.nodes = new Map();
    this.freqs = new Map();
  }
  _touch(node) {
    const f = node.freq;
    const dll = this.freqs.get(f);
    dll.remove(node);
    if (dll.size === 0) {
      this.freqs.delete(f);
      if (this.minFreq === f) this.minFreq++;
    }
    node.freq++;
    if (!this.freqs.has(node.freq)) this.freqs.set(node.freq, new DLL());
    this.freqs.get(node.freq).addFront(node);
  }
  get(key) {
    if (!this.nodes.has(key)) return null;
    const node = this.nodes.get(key);
    this._touch(node);
    return node.val;
  }
  set(key, value) {
    if (this.cap <= 0) return;
    if (this.nodes.has(key)) {
      const node = this.nodes.get(key);
      node.val = value;
      this._touch(node);
      return;
    }
    if (this.nodes.size >= this.cap) {
      const lru = this.freqs.get(this.minFreq).removeLast();
      this.nodes.delete(lru.key);
    }
    const n = new Node(key, value);
    this.nodes.set(key, n);
    if (!this.freqs.has(1)) this.freqs.set(1, new DLL());
    this.freqs.get(1).addFront(n);
    this.minFreq = 1;
  }
}

const fmt = (x) => (x === null ? "null" : String(x));

function main() {
  const cache = new LFUCache(2);
  cache.set(1, 1);
  cache.set(2, 2);
  console.log(fmt(cache.get(1))); // 1
  cache.set(3, 3); // evicts key 2
  console.log(fmt(cache.get(2))); // null
  cache.get(3); // access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
  cache.set(4, 4); // evicts key 1
  console.log(fmt(cache.get(1))); // null
  console.log(fmt(cache.get(3))); // 3
  console.log(fmt(cache.get(4))); // 4
}

main();
