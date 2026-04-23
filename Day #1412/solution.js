// Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: lazy nodes — children are materialized only on access (here via a deterministic
// LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.

let lcg = 42n;
function nextRand() {
  lcg = (lcg * 1103515245n + 12345n) & 0xffffffffffffffffn;
  return Number((lcg >> 16n) & 0x7fffn);
}

class LazyNode {
  constructor(depth) {
    this.depth = depth;
    this.leftDone = false;
    this.rightDone = false;
    this.leftCache = null;
    this.rightCache = null;
  }
  spawn() {
    const bound = 55 - this.depth * 7;
    return bound > 0 && nextRand() % 100 < bound;
  }
  left() {
    if (!this.leftDone) {
      this.leftDone = true;
      if (this.spawn()) this.leftCache = new LazyNode(this.depth + 1);
    }
    return this.leftCache;
  }
  right() {
    if (!this.rightDone) {
      this.rightDone = true;
      if (this.spawn()) this.rightCache = new LazyNode(this.depth + 1);
    }
    return this.rightCache;
  }
}

// generate(): O(1)
function generate() {
  return new LazyNode(0);
}

function countNodes(n) {
  if (n === null) return 0;
  const l = countNodes(n.left());
  const r = countNodes(n.right());
  return 1 + l + r;
}

const root = generate();
console.log("Tree size:", countNodes(root));
