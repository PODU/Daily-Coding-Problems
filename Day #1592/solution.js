// readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
// Time O(n) per readN call.
'use strict';

class Reader {
  constructor(file) {
    this.file = file;
    this.pos = 0;   // internal pointer for read7
    this.buf = '';  // leftover chars buffered between readN calls
  }

  // read7 primitive: up to 7 chars, advances pointer, "" at EOF
  read7() {
    const res = this.file.slice(this.pos, this.pos + 7);
    this.pos += res.length;
    return res;
  }

  // readN: read exactly n chars (or fewer at EOF), buffering leftovers
  readN(n) {
    let out = '';
    while (out.length < n) {
      if (this.buf.length === 0) {
        this.buf = this.read7();
        if (this.buf.length === 0) break; // EOF
      }
      const take = Math.min(n - out.length, this.buf.length);
      out += this.buf.slice(0, take);
      this.buf = this.buf.slice(take);
    }
    return out;
  }
}

const r1 = new Reader('Hello world');
console.log('read7: "' + r1.read7() + '"');
console.log('read7: "' + r1.read7() + '"');
console.log('read7: "' + r1.read7() + '"');

const r2 = new Reader('Hello world');
console.log('readN(5): "' + r2.readN(5) + '"');
console.log('readN(100): "' + r2.readN(100) + '"');
console.log('readN(3): "' + r2.readN(3) + '"');
