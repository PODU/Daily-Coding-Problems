// readN(n) on top of read7(): buffer leftover chars between calls.
// Time O(n) per readN call, Space O(1) extra (small buffer).
class Reader {
  constructor(content) {
    this.content = content;
    this.pos = 0; // read7 pointer
    this.buf = ""; // leftover unconsumed chars
  }

  read7() {
    const chunk = this.content.slice(this.pos, this.pos + 7);
    this.pos += chunk.length;
    return chunk;
  }

  readN(n) {
    while (this.buf.length < n) {
      const chunk = this.read7();
      if (chunk === "") break;
      this.buf += chunk;
    }
    const out = this.buf.slice(0, n);
    this.buf = this.buf.slice(n);
    return out;
  }
}

const r = new Reader("Hello world");
console.log('"' + r.readN(7) + '"');
console.log('"' + r.readN(7) + '"');
console.log('"' + r.readN(7) + '"');
