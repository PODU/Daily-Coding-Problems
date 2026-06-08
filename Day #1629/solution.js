// Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
// Time O(n), Space O(n).
function partition3(x, lst) {
  const less = [], equal = [], greater = [];
  for (const v of lst) {
    if (v < x) less.push(v);
    else if (v === x) equal.push(v);
    else greater.push(v);
  }
  return [...less, ...equal, ...greater];
}

const res = partition3(10, [9, 12, 3, 5, 14, 10, 10]);
console.log("[" + res.join(", ") + "]");
