// Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: root created in O(1); children expanded lazily with randomized termination
// (each child exists with prob < 0.5 => branching process is finite almost surely).
// generate(): O(1). Materializing whole tree: O(size). Deterministic demo via MINSTD RNG.
'use strict';

let rngState = 42;
function nextRand() {
  rngState = (rngState * 48271) % 2147483647;
  return rngState / 2147483647;
}

const P = 0.45;
const DEPTH_CAP = 50;

class Node {
  constructor() { this.left = null; this.right = null; this.expanded = false; }
}

function ensureChildren(n, depth) {
  if (n.expanded) return;
  n.expanded = true;
  if (depth >= DEPTH_CAP) return;
  if (nextRand() < P) n.left = new Node();
  if (nextRand() < P) n.right = new Node();
}

function generate() { return new Node(); } // O(1)

function countNodes(n, depth) {
  if (n === null) return 0;
  ensureChildren(n, depth);
  return 1 + countNodes(n.left, depth + 1) + countNodes(n.right, depth + 1);
}

const root = generate();
console.log(`Generated a finite binary tree with ${countNodes(root, 0)} nodes (deterministic demo).`);
