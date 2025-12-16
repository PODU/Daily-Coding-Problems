// generate() returns a root in O(1); children are materialized lazily on first access.
// Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
// generate(): O(1). Traversal materializes nodes on demand.

const P = 0.45;

// mulberry32 seeded PRNG for reproducible output.
function makeRng(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0; a = (a + 0x6D2B79F5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

class LazyNode {
  constructor(value, rng) {
    this.value = value;
    this.rng = rng;
    this._left = null; this.lSet = false;
    this._right = null; this.rSet = false;
  }
  left() {
    if (!this.lSet) { this.lSet = true; this._left = this.rng() < P ? new LazyNode(0, this.rng) : null; }
    return this._left;
  }
  right() {
    if (!this.rSet) { this.rSet = true; this._right = this.rng() < P ? new LazyNode(0, this.rng) : null; }
    return this._right;
  }
}

function generate(rng) { return new LazyNode(0, rng); } // O(1)

function treeSize(n) {
  if (n === null) return 0;
  return 1 + treeSize(n.left()) + treeSize(n.right());
}

const rng = makeRng(42);
const root = generate(rng); // O(1)
console.log("Generated finite tree size:", treeSize(root));
