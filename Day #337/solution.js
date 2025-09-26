// Shuffle linked list uniformly via Fisher-Yates on node values.
// O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.
'use strict';

class Node {
  constructor(val) { this.val = val; this.next = null; }
}

// Deterministic seeded PRNG (mulberry32).
function mulberry32(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0; a = (a + 0x6D2B79F5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function main() {
  let head = null, tail = null;
  for (let v = 1; v <= 5; ++v) {
    const n = new Node(v);
    if (!head) head = tail = n; else { tail.next = n; tail = n; }
  }
  const nodes = [];
  for (let p = head; p; p = p.next) nodes.push(p);
  const n = nodes.length;

  const rng = mulberry32(42);
  for (let i = n - 1; i > 0; --i) {
    const j = Math.floor(rng() * (i + 1));
    const t = nodes[i].val; nodes[i].val = nodes[j].val; nodes[j].val = t;
  }

  const out = [];
  for (let p = head; p; p = p.next) out.push(p.val);
  console.log(out.join(' '));
}

main();
