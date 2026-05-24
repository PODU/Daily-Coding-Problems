// Lazy binary tree: generate() returns a root in O(1) whose children are thunks
// (closures) forced on demand; a seeded coin flip (<1 continue prob) makes the tree
// finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.
class LCG {
  constructor(seed) { this.s = seed; }
  next() { this.s = (this.s * 16807) % 2147483647; return this.s; } // Park-Miller, exact in f64
  coin() { return this.next() % 100 < 45; } // child exists prob 0.45 -> finite
}

let counter = 0;

// makeNode/generate do NOT force children: O(1).
function makeNode(rng) {
  const node = { val: counter++ };
  node.left = () => (rng.coin() ? makeNode(rng) : null);
  node.right = () => (rng.coin() ? makeNode(rng) : null);
  return node;
}

function generate(rng) {
  return makeNode(rng); // O(1): one node, children unevaluated
}

function realize(node, depth, cap) {
  let count = 1;
  if (depth < cap) {
    const l = node.left();
    if (l) count += realize(l, depth + 1, cap);
    const r = node.right();
    if (r) count += realize(r, depth + 1, cap);
  }
  return count;
}

const rng = new LCG(42);
const root = generate(rng); // returns instantly
console.log("generate() returned a lazy tree root in O(1)");
const n = realize(root, 0, 6);
console.log("Realized tree node count: " + n);
