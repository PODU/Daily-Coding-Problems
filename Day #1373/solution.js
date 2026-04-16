// 2D iterator with lazy outer/inner pointers (no flatten/clone).
// next() & hasNext() amortized O(1), Space O(1) extra.
class Iterator2D {
  constructor(data) {
    this.data = data;
    this.outer = 0;
    this.inner = 0;
  }
  hasNext() {
    while (this.outer < this.data.length && this.inner >= this.data[this.outer].length) {
      this.outer++;
      this.inner = 0;
    }
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
console.log(out.join(", "));
