// Day 1584: 2D iterator over array of arrays (no flatten/clone).
// Maintain (outer,inner) indices; skip advances past empty inner arrays. Time: O(1) amortized; Space: O(1).
"use strict";

class Iterator2D {
  constructor(data) {
    this.data = data;
    this.outer = 0;
    this.inner = 0;
    this._skip();
  }
  _skip() {
    while (this.outer < this.data.length && this.inner >= this.data[this.outer].length) {
      this.outer++;
      this.inner = 0;
    }
  }
  hasNext() {
    this._skip();
    return this.outer < this.data.length;
  }
  next() {
    if (!this.hasNext()) throw new Error("no more elements");
    return this.data[this.outer][this.inner++];
  }
}

const it = new Iterator2D([[1, 2], [3], [], [4, 5, 6]]);
const out = [];
while (it.hasNext()) out.push(it.next());
console.log(out.join(", ")); // 1, 2, 3, 4, 5, 6
