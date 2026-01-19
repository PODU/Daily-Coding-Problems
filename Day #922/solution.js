// Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
// ancestors + descendant count. All ops O(h) where h is tree height.
'use strict';

class Node {
  constructor() {
    this.parent = null;
    this.left = null;
    this.right = null;
    this.locked = false;
    this.lockedDescendants = 0;
  }

  isLocked() {
    return this.locked;
  }

  _anyAncestorLocked() {
    for (let p = this.parent; p; p = p.parent) if (p.locked) return true;
    return false;
  }

  lock() {
    if (this.locked) return false;
    if (this.lockedDescendants > 0) return false;
    if (this._anyAncestorLocked()) return false;
    this.locked = true;
    for (let p = this.parent; p; p = p.parent) p.lockedDescendants++;
    return true;
  }

  unlock() {
    if (!this.locked) return false;
    this.locked = false;
    for (let p = this.parent; p; p = p.parent) p.lockedDescendants--;
    return true;
  }
}

function main() {
  const root = new Node(), a = new Node(), b = new Node(), c = new Node(), d = new Node();
  root.left = a; root.right = b;
  a.parent = root; b.parent = root;
  a.left = c; a.right = d;
  c.parent = a; d.parent = a;

  console.log(`lock c (leaf)      -> ${c.lock()} (expect true)`);
  console.log(`lock a (ancestor)  -> ${a.lock()} (expect false)`);
  console.log(`lock root          -> ${root.lock()} (expect false)`);
  console.log(`unlock c           -> ${c.unlock()} (expect true)`);
  console.log(`lock a             -> ${a.lock()} (expect true)`);
  console.log(`lock c (desc lock) -> ${c.lock()} (expect false)`);
  console.log(`unlock a           -> ${a.unlock()} (expect true)`);
}

main();
