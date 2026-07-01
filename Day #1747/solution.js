// Day 1747: All permutations of a list of digits.
// Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
// Time O(n * n!), space O(n) recursion (plus O(n!) for the output).
'use strict';

function permutations(nums) {
  const res = [];
  const used = new Array(nums.length).fill(false);
  const cur = [];

  function backtrack() {
    if (cur.length === nums.length) {
      res.push(cur.slice());
      return;
    }
    for (let i = 0; i < nums.length; i++) {
      if (used[i]) continue;
      used[i] = true;
      cur.push(nums[i]);
      backtrack();
      cur.pop();
      used[i] = false;
    }
  }

  backtrack();
  return res;
}

function main() {
  const nums = [1, 2, 3];
  const res = permutations(nums);
  const inner = res.map(p => '[' + p.join(',') + ']').join(',');
  console.log('[' + inner + ']');
}

main();
