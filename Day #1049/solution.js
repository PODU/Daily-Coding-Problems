// LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
// Avoids a 1e9-size counting array; memory scales with N, not value range. Else: external merge sort.

function radixSort(a) {
  let buf = new Array(a.length).fill(0);
  for (let shift = 0; shift < 32; shift += 8) {
    const count = new Array(256).fill(0);
    for (const v of a) count[(v >>> shift) & 0xff]++;
    let sum = 0;
    for (let i = 0; i < 256; i++) { const c = count[i]; count[i] = sum; sum += c; }
    for (const v of a) { const d = (v >>> shift) & 0xff; buf[count[d]++] = v; }
    [a, buf] = [buf, a];
  }
  return a;
}

const a = [829, 3, 1000000000, 42, 17, 999, 256, 0, 524287, 42];
console.log(radixSort(a).join(" "));
