// Count smaller elements to the right via Fenwick/BIT + coordinate compression.
// Time O(n log n), Space O(n).
class BIT {
  constructor(n) {
    this.t = new Array(n + 1).fill(0);
  }
  update(i, v) {
    for (; i < this.t.length; i += i & -i) this.t[i] += v;
  }
  query(i) {
    let s = 0;
    for (; i > 0; i -= i & -i) s += this.t[i];
    return s;
  }
}

function lowerBound(arr, key) {
  let lo = 0,
    hi = arr.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] < key) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

function countSmaller(a) {
  const n = a.length;
  const uniq = [...new Set(a)].sort((x, y) => x - y);
  const bit = new BIT(uniq.length);
  const res = new Array(n).fill(0);
  for (let i = n - 1; i >= 0; i--) {
    const rank = lowerBound(uniq, a[i]) + 1;
    res[i] = bit.query(rank - 1);
    bit.update(rank, 1);
  }
  return res;
}

function main() {
  const a = [3, 4, 9, 6, 1];
  const res = countSmaller(a);
  console.log("[" + res.join(", ") + "]");
}

main();
