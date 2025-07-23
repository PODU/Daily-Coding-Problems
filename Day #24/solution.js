// Binary tree locking: each node has a parent pointer and lockedDescendantCount.
// lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).
class Node {
  constructor(name, parent = null) {
    this.name = name;
    this.parent = parent;
    this.left = null;
    this.right = null;
    this.locked = false;
    this.lockedDescendantCount = 0;
  }

  isLocked() {
    return this.locked;
  }

  canLock() {
    if (this.lockedDescendantCount > 0) return false;
    for (let cur = this.parent; cur; cur = cur.parent) {
      if (cur.locked) return false;
    }
    return true;
  }

  lock() {
    if (this.locked || !this.canLock()) return false;
    this.locked = true;
    for (let cur = this.parent; cur; cur = cur.parent) cur.lockedDescendantCount++;
    return true;
  }

  unlock() {
    if (!this.locked) return false;
    this.locked = false;
    for (let cur = this.parent; cur; cur = cur.parent) cur.lockedDescendantCount--;
    return true;
  }
}

const root = new Node('root');
const node1 = new Node('node1', root);
const node2 = new Node('node2', root);
root.left = node1;
root.right = node2;
const node3 = new Node('node3', node1);
const node4 = new Node('node4', node1);
node1.left = node3;
node1.right = node4;

console.log('node2.lock() = ' + node2.lock());
console.log('root.lock() = ' + root.lock());
console.log('node2.unlock() = ' + node2.unlock());
console.log('root.lock() = ' + root.lock());
