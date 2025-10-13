// Day 421: LSD radix sort (base-256). O(n*w) time, O(n) space.
// 4 byte-passes for 32-bit ints; no billion-element array required.
function radixSort(a) {
  a = a.slice();
  for (let shift = 0; shift < 32; shift += 8) {
    const cnt = new Array(257).fill(0);
    for (const x of a) cnt[((x >>> shift) & 0xFF) + 1]++;
    for (let i = 0; i < 256; i++) cnt[i + 1] += cnt[i];
    const buf = new Array(a.length);
    for (const x of a) {
      const d = (x >>> shift) & 0xFF;
      buf[cnt[d]++] = x;
    }
    a = buf;
  }
  return a;
}

const a = [5, 3, 1000000000, 0, 42, 7, 42];
console.log("Sorted: [" + radixSort(a).join(", ") + "]");
