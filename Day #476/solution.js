// Find duplicate in array of n+1 ints from 1..n using a Set.
// Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
function findDuplicate(a) {
  const seen = new Set();
  for (const x of a) {
    if (seen.has(x)) return x;
    seen.add(x);
  }
  return -1;
}

console.log(findDuplicate([1, 3, 4, 2, 2]));
