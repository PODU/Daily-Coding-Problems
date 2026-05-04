// Day 1471: O(log N) search in a sorted array with no *, /, or bit-shift.
// Build powers of two by doubling (addition); jump-based binary search.
// Time O(log N), Space O(log N).

function search(a, x) {
  const n = a.length;
  if (n === 0) return false;
  const powers = [];
  for (let p = 1; p <= n; p = p + p) powers.push(p);
  let pos = -1;
  for (let i = powers.length - 1; i >= 0; --i) {
    const nxt = pos + powers[i];
    if (nxt < n && a[nxt] <= x) pos = nxt;
  }
  return pos >= 0 && a[pos] === x;
}

const arr = [1, 3, 5, 7, 9, 11];
console.log(search(arr, 7) ? "True" : "False");
console.log(search(arr, 8) ? "True" : "False");
