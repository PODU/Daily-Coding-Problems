// Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
// Time O(1) per op; Space O(1).

class SimpleIterator {
  constructor(data) {
    this.data = data;
    this.idx = 0;
  }
  next() {
    return this.data[this.idx++];
  }
  hasNext() {
    return this.idx < this.data.length;
  }
}

class PeekableInterface {
  constructor(iterator) {
    this.it = iterator;
    this.buffered = false;
    this.buffer = undefined;
  }
  peek() {
    if (!this.buffered) {
      this.buffer = this.it.next();
      this.buffered = true;
    }
    return this.buffer;
  }
  next() {
    if (this.buffered) {
      this.buffered = false;
      return this.buffer;
    }
    return this.it.next();
  }
  hasNext() {
    return this.buffered || this.it.hasNext();
  }
}

const p = new PeekableInterface(new SimpleIterator([1, 2, 3]));
console.log(
  `peek=${p.peek()} next=${p.next()} peek=${p.peek()} next=${p.next()} next=${p.next()} hasNext=${p.hasNext()}`
);
// peek=1 next=1 peek=2 next=2 next=3 hasNext=false
