// Dutch National Flag 3-way partition (R<G<B). In-place, O(n) time, O(1) space, swaps only.
function sortRGB(a) {
  let low = 0, mid = 0, high = a.length - 1;
  while (mid <= high) {
    if (a[mid] === 'R') { [a[low], a[mid]] = [a[mid], a[low]]; low++; mid++; }
    else if (a[mid] === 'G') { mid++; }
    else { [a[mid], a[high]] = [a[high], a[mid]]; high--; }
  }
  return a;
}

const a = ['G', 'B', 'R', 'R', 'B', 'R', 'G'];
sortRGB(a);
console.log("[" + a.map(c => "'" + c + "'").join(", ") + "]");
