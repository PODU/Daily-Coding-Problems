// Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
class Peekable {
  constructor(iterable) {
    this._it = iterable[Symbol.iterator]();
    this._cached = null;
    this._hasCached = false;
  }
  hasNext() {
    if (this._hasCached) return true;
    const r = this._it.next();
    if (r.done) return false;
    this._cached = r.value;
    this._hasCached = true;
    return true;
  }
  peek() {
    if (!this._hasCached) {
      const r = this._it.next();
      this._cached = r.value;
      this._hasCached = true;
    }
    return this._cached;
  }
  next() {
    const val = this.peek();
    this._hasCached = false;
    this._cached = null;
    return val;
  }
}

const p = new Peekable([1, 2, 3]);
console.log("peek()    ->", p.peek());
console.log("next()    ->", p.next());
console.log("peek()    ->", p.peek());
console.log("hasNext() ->", p.hasNext());
console.log("next()    ->", p.next());
console.log("next()    ->", p.next());
console.log("hasNext() ->", p.hasNext());
