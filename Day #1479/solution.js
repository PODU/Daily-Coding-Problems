// Day 1479: Partition a list into <x, ==x, >x around pivot x.
// Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).

function partition(lst, x) {
  const less = [], equal = [], greater = [];
  for (const v of lst) {
    if (v < x) less.push(v);
    else if (v === x) equal.push(v);
    else greater.push(v);
  }
  return [...less, ...equal, ...greater];
}

console.log(partition([9, 12, 3, 5, 14, 10, 10], 10));
// [ 9, 3, 5, 10, 10, 12, 14 ]
