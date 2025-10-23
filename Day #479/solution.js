// Generate all permutations via backtracking, picking remaining elements in
// index order so output is lexicographic. Time: O(n! * n), Space: O(n) recursion.

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
  console.log("[" + res.map((p) => "[" + p.join(",") + "]").join(",") + "]");
}

main();
