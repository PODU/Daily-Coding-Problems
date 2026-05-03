// All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
// Time O(n! * n), Space O(n) recursion + output.
function permute(nums) {
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

const res = permute([1, 2, 3]);
const out = "[" + res.map((p) => "[" + p.join(",") + "]").join(",") + "]";
console.log(out);
