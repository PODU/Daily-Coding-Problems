// Peekable iterator: cache one element ahead so peek() returns next without advancing.
// next/peek/hasNext all O(1).

class Peekable {
  constructor(iterable) {
    this.it = iterable[Symbol.iterator]();
    this.cached = null; // { value }
  }
  hasNext() {
    if (!this.cached) {
      const r = this.it.next();
      this.cached = r.done ? null : { value: r.value };
    }
    return this.cached !== null;
  }
  peek() {
    if (!this.cached) {
      const r = this.it.next();
      this.cached = r.done ? null : { value: r.value };
    }
    return this.cached.value;
  }
  next() {
    if (this.cached) {
      const v = this.cached.value;
      this.cached = null;
      return v;
    }
    return this.it.next().value;
  }
}

const it = new Peekable([1, 2, 3, 4]);
console.log(it.peek());    // 1
console.log(it.next());    // 1
console.log(it.next());    // 2
console.log(it.peek());    // 3
console.log(it.next());    // 3
console.log(it.hasNext()); // true
console.log(it.next());    // 4
console.log(it.hasNext()); // false
