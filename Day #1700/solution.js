// Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
// On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
class Quack {
  constructor() {
    this.L = []; // top = leftmost
    this.R = []; // top = rightmost
    this.T = []; // transient buffer
  }
  push(x) { this.L.push(x); } // add to LEFT end
  pop() { // remove LEFT end
    if (this.L.length === 0) this._rebalanceLfromR();
    return this.L.pop();
  }
  pull() { // remove RIGHT end
    if (this.R.length === 0) this._rebalanceRfromL();
    return this.R.pop();
  }
  _rebalanceLfromR() {
    const size = this.R.length;
    let half = Math.floor(size / 2) || 1;
    const keep = size - half;
    for (let k = 0; k < keep; k++) this.T.push(this.R.pop());
    while (this.R.length) this.L.push(this.R.pop());
    while (this.T.length) this.R.push(this.T.pop());
  }
  _rebalanceRfromL() {
    const size = this.L.length;
    let half = Math.floor(size / 2) || 1;
    const keep = size - half;
    for (let k = 0; k < keep; k++) this.T.push(this.L.pop());
    while (this.L.length) this.R.push(this.L.pop());
    while (this.T.length) this.L.push(this.T.pop());
  }
}

const q = new Quack();
q.push(1); q.push(2); q.push(3); // left-to-right: 3,2,1
console.log(q.pop());   // 3
console.log(q.pull());  // 1
q.push(4);              // now 4,2
console.log(q.pop());   // 4
console.log(q.pull());  // 2
