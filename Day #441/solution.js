// Day 441: Three-way partition around pivot x into <x, ==x, >x buckets.
// O(n) time, O(n) space (stable bucket order matches the example).

function partitionThree(lst, x) {
  const less = [], equal = [], greater = [];
  for (const v of lst) {
    if (v < x) less.push(v);
    else if (v === x) equal.push(v);
    else greater.push(v);
  }
  return [...less, ...equal, ...greater];
}

console.log(partitionThree([9, 12, 3, 5, 14, 10, 10], 10));
// [ 9, 3, 5, 10, 10, 12, 14 ]
