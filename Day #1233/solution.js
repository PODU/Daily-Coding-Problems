// Dutch national flag: one-pass 3-way partition. Time O(n), Space O(1).
function sortRGB(a) {
  let low = 0, mid = 0, high = a.length - 1;
  while (mid <= high) {
    if (a[mid] === 'R') { [a[low], a[mid]] = [a[mid], a[low]]; low++; mid++; }
    else if (a[mid] === 'G') mid++;
    else { [a[mid], a[high]] = [a[high], a[mid]]; high--; }
  }
  return a;
}

const arr = ['G', 'B', 'R', 'R', 'B', 'R', 'G'];
console.log("[" + sortRGB(arr).map((c) => `'${c}'`).join(", ") + "]");
