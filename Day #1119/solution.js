// Day 1119 - Split array into k contiguous partitions minimizing the max sum
// Binary search the answer; greedily count partitions. Time: O(n log(sum)).

function splitMinMax(N, k) {
  const partitionsNeeded = (limit) => {
    let count = 1, cur = 0;
    for (const x of N) {
      if (cur + x > limit) { count++; cur = x; }
      else cur += x;
    }
    return count;
  };

  let lo = Math.max(...N);
  let hi = N.reduce((a, b) => a + b, 0);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (partitionsNeeded(mid) <= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}

console.log(splitMinMax([5, 1, 2, 7, 3, 4], 3)); // 8
