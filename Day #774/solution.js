// Day 774: Implement readN(n) on top of a read7() primitive.
// Buffer leftover chars from read7 between calls. O(n) per readN call.
class Reader {
  constructor(file) { this.file = file; this.pos = 0; this.buf = ""; }

  read7() {
    const r = this.file.slice(this.pos, this.pos + 7);
    this.pos += r.length;
    return r;
  }

  readN(n) {
    let out = "";
    while (out.length < n) {
      if (this.buf.length === 0) {
        this.buf = this.read7();
        if (this.buf.length === 0) break;
      }
      const take = Math.min(this.buf.length, n - out.length);
      out += this.buf.slice(0, take);
      this.buf = this.buf.slice(take);
    }
    return out;
  }
}

const r = new Reader("Hello world");
console.log(`"${r.readN(7)}", "${r.readN(7)}", "${r.readN(7)}"`);
// "Hello w", "orld", ""
