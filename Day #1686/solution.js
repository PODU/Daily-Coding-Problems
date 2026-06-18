// Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
// reverse [i..m] to bring it to front. Time O(n^2), Space O(1).

function reverse(lst, i, j) {
  while (i < j) {
    const t = lst[i]; lst[i] = lst[j]; lst[j] = t;
    i++; j--;
  }
}

function sortWithReverse(lst) {
  const n = lst.length;
  for (let i = 0; i < n; i++) {
    let m = i;
    for (let k = i + 1; k < n; k++) if (lst[k] < lst[m]) m = k;
    reverse(lst, i, m);
  }
}

const data = [3, 1, 2, 5, 4];
sortWithReverse(data);
console.log(data.join(" "));
