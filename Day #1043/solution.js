// PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
// the next value without consuming it. O(1) per op, O(1) extra space.

class PeekableInterface {
  constructor(iterable) {
    this._it = iterable[Symbol.iterator]();
    this._cached = undefined;
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
  next() {
    if (this._hasCached) {
      this._hasCached = false;
      return this._cached;
    }
    return this._it.next().value;
  }
  peek() {
    if (!this._hasCached) {
      this._cached = this._it.next().value;
      this._hasCached = true;
    }
    return this._cached;
  }
}

function main() {
  const it = new PeekableInterface([1, 2, 3]);
  console.log(`peek() -> ${it.peek()}`);
  console.log(`next() -> ${it.next()}`);
  console.log(`peek() -> ${it.peek()}`);
  console.log(`next() -> ${it.next()}`);
  console.log(`next() -> ${it.next()}`);
  console.log(`hasNext() -> ${it.hasNext()}`);
}

main();
