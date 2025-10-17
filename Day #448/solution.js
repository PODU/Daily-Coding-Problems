// Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
// with three pointers (low=R boundary, high=B boundary, mid=scanner).

function sortRGB(a) {
  let low = 0, mid = 0, high = a.length - 1;
  while (mid <= high) {
    if (a[mid] === 'R') { [a[low], a[mid]] = [a[mid], a[low]]; low++; mid++; }
    else if (a[mid] === 'G') { mid++; }
    else { [a[mid], a[high]] = [a[high], a[mid]]; high--; }
  }
  return a;
}

console.log(sortRGB(['G', 'B', 'R', 'R', 'B', 'R', 'G']));
// [ 'R', 'R', 'R', 'G', 'G', 'B', 'B' ]
