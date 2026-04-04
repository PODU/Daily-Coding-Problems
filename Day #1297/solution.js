// Day 1297: Implement readN(n) on top of a read7() primitive.
// Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.
class Reader {
  constructor(content) {
    this.file = content;
    this.pos = 0;
  }
  read7() {
    // up to 7 chars, "" at EOF
    const r = this.file.slice(this.pos, this.pos + 7);
    this.pos += r.length;
    return r;
  }
}

class NReader {
  constructor(reader) {
    this.r = reader;
    this.buf = "";
  }
  readN(n) {
    while (this.buf.length < n) {
      const chunk = this.r.read7();
      if (chunk === "") break;
      this.buf += chunk;
    }
    const out = this.buf.slice(0, n);
    this.buf = this.buf.slice(n);
    return out;
  }
}

const nr = new NReader(new Reader("Hello world"));
console.log(`'${nr.readN(5)}'`);  // 'Hello'
console.log(`'${nr.readN(4)}'`);  // ' wor'
console.log(`'${nr.readN(10)}'`); // 'ld'
