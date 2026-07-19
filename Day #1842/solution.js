// Day 1842: Majority / most-frequent element via a frequency count.
// (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).

function majority(a) {
  const freq = new Map();
  let best = a[0],
    bestCount = 0;
  for (const x of a) {
    const c = (freq.get(x) || 0) + 1;
    freq.set(x, c);
    if (c > bestCount) {
      bestCount = c;
      best = x;
    }
  }
  return best;
}

function main() {
  console.log(majority([1, 2, 1, 1, 3, 4, 0])); // 1
}

main();
