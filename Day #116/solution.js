// Day 116: generate() returns a root in O(1); children materialize lazily on access.
// Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
// (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
class Node {
  constructor(depth) {
    this.depth = depth;
    this.l = null;
    this.r = null;
  }
}

let lcg = 42;
function nextRand() {
  lcg = (lcg * 16807) % 2147483647;
  return lcg;
}
const threshold = (d) => Math.max(0, 750 - 80 * d);

function generate() {
  return new Node(0); // O(1)
}

function force(n) {
  let cnt = 1;
  if (nextRand() % 1000 < threshold(n.depth)) { n.l = new Node(n.depth + 1); cnt += force(n.l); }
  if (nextRand() % 1000 < threshold(n.depth)) { n.r = new Node(n.depth + 1); cnt += force(n.r); }
  return cnt;
}

const root = generate();
const n = force(root);
console.log(`Generated a finite binary tree with ${n} nodes`);
