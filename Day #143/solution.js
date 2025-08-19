// Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.

function partition(lst, x) {
  const less = [], equal = [], greater = [];
  for (const v of lst) {
    if (v < x) less.push(v);
    else if (v === x) equal.push(v);
    else greater.push(v);
  }
  return [...less, ...equal, ...greater];
}

const lst = [9, 12, 3, 5, 14, 10, 10];
console.log("[" + partition(lst, 10).join(", ") + "]"); // [9, 3, 5, 10, 10, 12, 14]
