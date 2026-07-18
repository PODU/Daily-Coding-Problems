// Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
// Time O(N log N), Space O(N).

function countSmaller(a) {
  const uniq = [...new Set(a)].sort((x, y) => x - y);
  const tree = new Array(uniq.length + 1).fill(0);
  const add = (i) => {
    for (; i < tree.length; i += i & -i) tree[i]++;
  };
  const sum = (i) => {
    let s = 0;
    for (; i > 0; i -= i & -i) s += tree[i];
    return s;
  };
  const lowerBound = (x) => {
    let lo = 0,
      hi = uniq.length;
    while (lo < hi) {
      const m = (lo + hi) >> 1;
      if (uniq[m] < x) lo = m + 1;
      else hi = m;
    }
    return lo;
  };
  const res = new Array(a.length);
  for (let i = a.length - 1; i >= 0; i--) {
    const r = lowerBound(a[i]) + 1;
    res[i] = sum(r - 1);
    add(r);
  }
  return res;
}

function main() {
  console.log(JSON.stringify(countSmaller([3, 4, 9, 6, 1])));
}

main();
