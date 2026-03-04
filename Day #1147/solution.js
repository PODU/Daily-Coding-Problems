// Day 1147: Locking in a binary tree.
// Node keeps parent + count of locked descendants; lock/unlock walk ancestors. O(h).
class Node {
  constructor(parent = null) {
    this.left = null;
    this.right = null;
    this.parent = parent;
    this.locked = false;
    this.lockedDesc = 0;
  }
  isLocked() { return this.locked; }
  _canLock() {
    if (this.locked || this.lockedDesc > 0) return false;
    for (let a = this.parent; a; a = a.parent) if (a.locked) return false;
    return true;
  }
  lock() {
    if (!this._canLock()) return false;
    this.locked = true;
    for (let a = this.parent; a; a = a.parent) a.lockedDesc++;
    return true;
  }
  unlock() {
    if (!this.locked) return false;
    this.locked = false;
    for (let a = this.parent; a; a = a.parent) a.lockedDesc--;
    return true;
  }
}

const root = new Node();
const l = new Node(root), r = new Node(root), ll = new Node(l);
root.left = l; root.right = r; l.left = ll;
console.log(l.lock());    // true
console.log(ll.lock());   // false
console.log(root.lock()); // false
console.log(l.unlock());  // true
console.log(ll.lock());   // true
