// Tree node with parent pointer + lockedDescendants count; lock/unlock walk ancestors O(h).
// isLocked O(1). lock/unlock O(h). Space O(n).
'use strict';

class Node {
  constructor(parent = null) {
    this.parent = parent;
    this.left = null;
    this.right = null;
    this.locked = false;
    this.lockedDescendants = 0;
  }

  isLocked() { return this.locked; }

  anyAncestorLocked() {
    for (let p = this.parent; p; p = p.parent) if (p.locked) return true;
    return false;
  }

  lock() {
    if (this.locked || this.lockedDescendants > 0 || this.anyAncestorLocked()) return false;
    this.locked = true;
    for (let p = this.parent; p; p = p.parent) p.lockedDescendants++;
    return true;
  }

  unlock() {
    if (!this.locked || this.lockedDescendants > 0 || this.anyAncestorLocked()) return false;
    this.locked = false;
    for (let p = this.parent; p; p = p.parent) p.lockedDescendants--;
    return true;
  }
}

const root = new Node();
root.left = new Node(root);
root.right = new Node(root);
root.left.left = new Node(root.left);
root.left.right = new Node(root.left);
const L = root.left;
const LL = root.left.left;

console.log(LL.lock());
console.log(L.lock());
console.log(root.lock());
console.log(LL.unlock());
console.log(L.lock());
console.log(root.lock());
