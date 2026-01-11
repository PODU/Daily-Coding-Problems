// 2D iterator over array of arrays without flattening. next/hasNext amortized O(1).

class Iterator2D {
  constructor(data) {
    this.data = data;
    this.row = 0;
    this.col = 0;
  }
  _advance() {
    while (this.row < this.data.length && this.col >= this.data[this.row].length) {
      this.row++;
      this.col = 0;
    }
  }
  hasNext() {
    this._advance();
    return this.row < this.data.length;
  }
  next() {
    if (!this.hasNext()) throw new Error("no more elements");
    return this.data[this.row][this.col++];
  }
}

const it = new Iterator2D([[1, 2], [3], [], [4, 5, 6]]);
const out = [];
while (it.hasNext()) out.push(it.next());
console.log(out.join(", "));
