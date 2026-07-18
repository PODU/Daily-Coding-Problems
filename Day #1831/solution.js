// Smallest positive int not a subset sum of a sorted array. Greedy O(N).
// Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
function smallestMissing(a) {
  let res = 1;
  for (const x of a) {
    if (x > res) break;
    res += x;
  }
  return res;
}

console.log(smallestMissing([1, 2, 3, 10])); // 7
