// Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
// (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
// 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
// If even the million don't fit, fall back to external merge sort (chunk-sort to
// disk, then k-way merge).
function radixSort(input) {
  let a = input.slice();
  let tmp = new Array(a.length);
  for (let shift = 0; shift < 32; shift += 8) {
    const count = new Array(256).fill(0);
    for (const x of a) count[(x >>> shift) & 0xff]++;
    let sum = 0;
    for (let i = 0; i < 256; i++) { const c = count[i]; count[i] = sum; sum += c; }
    for (const x of a) tmp[count[(x >>> shift) & 0xff]++] = x;
    [a, tmp] = [tmp, a];
  }
  return a;
}

const data = [999999999, 0, 123456789, 42, 1000000000, 7, 500000000];
console.log(`[${radixSort(data).join(", ")}]`);
