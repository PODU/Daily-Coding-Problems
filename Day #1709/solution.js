// Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
function checkPossibility(arr) {
  const a = arr.slice();
  let cnt = 0;
  for (let i = 1; i < a.length; i++) {
    if (a[i - 1] > a[i]) {
      if (++cnt > 1) return false;
      if (i < 2 || a[i - 2] <= a[i]) a[i - 1] = a[i];
      else a[i] = a[i - 1];
    }
  }
  return true;
}

console.log(checkPossibility([10, 5, 7]));
console.log(checkPossibility([10, 5, 1]));
