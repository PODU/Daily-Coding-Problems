// Day 575: 2D iterator over an array of arrays without flattening/cloning.
// Track (row, col); advance over empty rows. next/hasNext amortized O(1).
'use strict';

class Iterator2D {
  constructor(data) {
    this.data = data;
    this.row = 0;
    this.col = 0;
    this._skipEmpty();
  }
  _skipEmpty() {
    while (this.row < this.data.length && this.col >= this.data[this.row].length) {
      this.row++;
      this.col = 0;
    }
  }
  hasNext() {
    return this.row < this.data.length;
  }
  next() {
    if (!this.hasNext()) throw new Error('no more elements');
    const v = this.data[this.row][this.col++];
    this._skipEmpty();
    return v;
  }
}

const it = new Iterator2D([[1, 2], [3], [], [4, 5, 6]]);
const out = [];
while (it.hasNext()) out.push(it.next());
console.log(out.join(', ')); // 1, 2, 3, 4, 5, 6
