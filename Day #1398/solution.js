// Single scan: count drops; on a drop, greedily fix by lowering a[i] or
// raising a[i+1] depending on a[i-1]. >1 drop => false. Time O(n), Space O(1).

function checkPossibility(arr) {
  const a = arr.slice();
  let cnt = 0;
  for (let i = 0; i + 1 < a.length; i++) {
    if (a[i] > a[i + 1]) {
      if (++cnt > 1) return false;
      if (i === 0 || a[i - 1] <= a[i + 1]) a[i] = a[i + 1];
      else a[i + 1] = a[i];
    }
  }
  return true;
}

console.log(checkPossibility([10, 5, 7]));
console.log(checkPossibility([10, 5, 1]));
