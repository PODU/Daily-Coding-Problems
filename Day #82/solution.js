// Day 82: readN(n) built on read7() by buffering leftover characters between calls.
// Time O(n) per call amortized, Space O(7) buffer.
class Reader {
  constructor(file) {
    this.file = file;
    this.pos = 0;
    this.buffer = "";
  }

  // Returns up to 7 characters from the file, advancing the cursor.
  read7() {
    const chunk = this.file.slice(this.pos, this.pos + 7);
    this.pos += chunk.length;
    return chunk;
  }

  readN(n) {
    let result = "";
    while (result.length < n) {
      if (this.buffer.length === 0) {
        const chunk = this.read7();
        if (chunk.length === 0) break; // EOF
        this.buffer = chunk;
      }
      const take = Math.min(this.buffer.length, n - result.length);
      result += this.buffer.slice(0, take);
      this.buffer = this.buffer.slice(take);
    }
    return result;
  }
}

const r = new Reader("Hello world");
console.log(`"${r.readN(7)}"`); // "Hello w"
console.log(`"${r.readN(7)}"`); // "orld"
console.log(`"${r.readN(7)}"`); // ""
