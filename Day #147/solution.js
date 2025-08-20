// Pancake sort: only primitive is reverse(lst,i,j). Each round reverse the window's max into place. O(n^2) time, O(1) space.

function reverse(lst, i, j) {
  while (i < j) {
    [lst[i], lst[j]] = [lst[j], lst[i]];
    i++;
    j--;
  }
}

function pancakeSort(lst) {
  const n = lst.length;
  for (let size = n; size > 1; size--) {
    let maxIdx = 0;
    for (let k = 1; k < size; k++) if (lst[k] > lst[maxIdx]) maxIdx = k;
    if (maxIdx !== size - 1) reverse(lst, maxIdx, size - 1);
  }
  return lst;
}

const a = [3, 6, 1, 5, 2, 4];
console.log(pancakeSort(a)); // [ 1, 2, 3, 4, 5, 6 ]
