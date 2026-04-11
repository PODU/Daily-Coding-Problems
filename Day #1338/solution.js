// Longest consecutive run via hash set: start only at run heads (x-1 absent), walk up. O(n) time, O(n) space.

function longestConsecutive(nums) {
  const set = new Set(nums);
  let best = 0;
  for (const x of set) {
    if (set.has(x - 1)) continue; // not a run head
    let cur = x, len = 1;
    while (set.has(cur + 1)) { cur++; len++; }
    best = Math.max(best, len);
  }
  return best;
}

function main() {
  const nums = [100, 4, 200, 1, 3, 2];
  console.log(longestConsecutive(nums));
}

main();
