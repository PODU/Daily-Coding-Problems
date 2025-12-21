// Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.
function countInversions(arr) {
  function sortCount(a) {
    if (a.length <= 1) return [a, 0];
    const mid = a.length >> 1;
    const [left, cl] = sortCount(a.slice(0, mid));
    const [right, cr] = sortCount(a.slice(mid));
    const merged = [];
    let i = 0, j = 0, c = cl + cr;
    while (i < left.length && j < right.length) {
      if (left[i] <= right[j]) merged.push(left[i++]);
      else { merged.push(right[j++]); c += left.length - i; }
    }
    while (i < left.length) merged.push(left[i++]);
    while (j < right.length) merged.push(right[j++]);
    return [merged, c];
  }
  return sortCount(arr)[1];
}

console.log(countInversions([2, 4, 1, 3, 5])); // 3
console.log(countInversions([5, 4, 3, 2, 1])); // 10
