// Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
// O(n^2) comparisons, O(n) reversals, in place. Space O(1).
function reverse(lst, i, j) {
  while (i < j) { [lst[i], lst[j]] = [lst[j], lst[i]]; i++; j--; }
}

function pancakeSort(lst) {
  const n = lst.length;
  for (let end = n - 1; end > 0; end--) {
    let mi = 0;
    for (let k = 1; k <= end; k++) if (lst[k] > lst[mi]) mi = k;
    if (mi === end) continue;
    if (mi !== 0) reverse(lst, 0, mi); // bring max to front
    reverse(lst, 0, end);              // move max to its final position
  }
  return lst;
}

const arr = [3, 1, 4, 1, 5, 9, 2, 6];
pancakeSort(arr);
console.log("[" + arr.join(", ") + "]");
