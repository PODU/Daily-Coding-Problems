// Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
// Index by element count, not by value. LSD radix sort (base 256) is O(N).
// For out-of-core data the same idea generalizes to external merge sort.
// Time O(N) (4 passes for 32-bit), Space O(N).

function radixSort(arr) {
  if (arr.length === 0) return arr;
  let out = arr.slice();
  let tmp = new Array(out.length);
  for (let shift = 0; shift < 32; shift += 8) {
    const count = new Array(257).fill(0);
    for (const v of out) count[((v >>> shift) & 0xff) + 1]++;
    for (let i = 0; i < 256; ++i) count[i + 1] += count[i];
    for (const v of out) {
      const d = (v >>> shift) & 0xff;
      tmp[count[d]++] = v;
    }
    [out, tmp] = [tmp, out];
  }
  return out;
}

console.log(radixSort([9, 11, 8, 5, 7, 10])); // [ 5, 7, 8, 9, 10, 11 ]
