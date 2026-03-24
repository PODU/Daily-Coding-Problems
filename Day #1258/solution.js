// Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.

class SimpleIterator {
  constructor(arr) { this.data = arr; this.idx = 0; }
  next() { return this.data[this.idx++]; }
  hasNext() { return this.idx < this.data.length; }
}

class PeekableInterface {
  constructor(iterator) { this.it = iterator; this.buffer = null; this.hasBuffer = false; }
  peek() {
    if (!this.hasBuffer) { this.buffer = this.it.next(); this.hasBuffer = true; }
    return this.buffer;
  }
  next() {
    if (this.hasBuffer) { this.hasBuffer = false; return this.buffer; }
    return this.it.next();
  }
  hasNext() { return this.hasBuffer || this.it.hasNext(); }
}

function main() {
  const p = new PeekableInterface(new SimpleIterator([1, 2, 3]));
  console.log(p.peek());
  console.log(p.next());
  console.log(p.next());
  console.log(p.peek());
  console.log(p.hasNext());
  console.log(p.next());
  console.log(p.hasNext());
}

main();
